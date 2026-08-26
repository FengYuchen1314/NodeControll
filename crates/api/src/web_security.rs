use std::net::{IpAddr, SocketAddr};

use axum::http::{HeaderMap, HeaderName, header};
use nodecontroll_application::ClientNetwork;
use nodecontroll_config::{PublicOrigin, TrustedProxyCidr};
use nodecontroll_identity::{CsrfToken, SessionToken, constant_time_bounded_string_equal};
use zeroize::Zeroizing;

pub const SESSION_COOKIE_NAME: &str = "__Host-nodecontroll_session";
pub const CSRF_COOKIE_NAME: &str = "__Host-nodecontroll_csrf";
pub const CSRF_HEADER_NAME: &str = "x-nodecontroll-csrf";

const MAX_FORWARDING_HEADER_BYTES: usize = 1_024;
const MAX_FORWARDED_HOPS: usize = 16;
const MAX_COOKIE_HEADER_BYTES: usize = 8_192;
const MAX_COOKIE_PAIRS: usize = 64;
const MAX_SECURITY_COOKIE_BYTES: usize = 96;
const MAX_USER_AGENT_BYTES: usize = 512;

#[derive(Clone)]
pub struct WebSecurityPolicy {
    public_origin: PublicOrigin,
    expected_host: String,
    trusted_proxies: Vec<TrustedProxyCidr>,
}

impl WebSecurityPolicy {
    #[must_use]
    pub fn new(public_origin: PublicOrigin, trusted_proxies: Vec<TrustedProxyCidr>) -> Self {
        let expected_host = public_origin
            .as_str()
            .split_once("://")
            .map(|(_, authority)| authority)
            .unwrap_or_default()
            .to_owned();
        Self {
            public_origin,
            expected_host,
            trusted_proxies,
        }
    }

    #[must_use]
    pub fn public_origin(&self) -> &str {
        self.public_origin.as_str()
    }

    pub fn validate_browser_origin(&self, headers: &HeaderMap) -> Result<(), WebSecurityError> {
        let origin =
            unique_header(headers, header::ORIGIN)?.ok_or(WebSecurityError::OriginMissing)?;
        if origin != self.public_origin.as_str() {
            return Err(WebSecurityError::OriginMismatch);
        }
        let host = unique_header(headers, header::HOST)?.ok_or(WebSecurityError::HostMissing)?;
        if !host.eq_ignore_ascii_case(&self.expected_host) {
            return Err(WebSecurityError::HostMismatch);
        }
        Ok(())
    }

    pub fn validate_request_host(&self, headers: &HeaderMap) -> Result<(), WebSecurityError> {
        let host = unique_header(headers, header::HOST)?.ok_or(WebSecurityError::HostMissing)?;
        if !host.eq_ignore_ascii_case(&self.expected_host) {
            return Err(WebSecurityError::HostMismatch);
        }
        Ok(())
    }

    pub fn resolve_client_network(
        &self,
        peer: SocketAddr,
        headers: &HeaderMap,
    ) -> Result<ClientNetwork, WebSecurityError> {
        let peer_ip = peer.ip();
        if !self.is_trusted_proxy(peer_ip) {
            return Ok(ClientNetwork::from_client_ip(peer_ip));
        }

        let forwarded = parse_x_forwarded_for(headers)?;
        if forwarded.is_empty() {
            return Err(WebSecurityError::InvalidForwardedChain);
        }
        let mut resolved = peer_ip;
        for candidate in forwarded.into_iter().rev() {
            if !self.is_trusted_proxy(resolved) {
                break;
            }
            resolved = candidate;
        }
        Ok(ClientNetwork::from_client_ip(resolved))
    }

    fn is_trusted_proxy(&self, address: IpAddr) -> bool {
        self.trusted_proxies
            .iter()
            .any(|network| network.contains(address))
    }
}

pub fn bounded_user_agent(headers: &HeaderMap) -> Result<String, WebSecurityError> {
    let value = unique_header(headers, header::USER_AGENT)?.unwrap_or_default();
    if value.len() > MAX_USER_AGENT_BYTES {
        return Err(WebSecurityError::InvalidSecurityHeader);
    }
    Ok(value.to_owned())
}

fn unique_header(
    headers: &HeaderMap,
    name: header::HeaderName,
) -> Result<Option<&str>, WebSecurityError> {
    let mut values = headers.get_all(name).iter();
    let Some(first) = values.next() else {
        return Ok(None);
    };
    if values.next().is_some() {
        return Err(WebSecurityError::DuplicateSecurityHeader);
    }
    first
        .to_str()
        .map(Some)
        .map_err(|_| WebSecurityError::InvalidSecurityHeader)
}

fn parse_x_forwarded_for(headers: &HeaderMap) -> Result<Vec<IpAddr>, WebSecurityError> {
    let mut encoded_bytes = 0_usize;
    let mut addresses = Vec::new();
    for value in headers.get_all("x-forwarded-for") {
        let value = value
            .to_str()
            .map_err(|_| WebSecurityError::InvalidForwardedChain)?;
        encoded_bytes = encoded_bytes
            .checked_add(value.len())
            .ok_or(WebSecurityError::InvalidForwardedChain)?;
        if encoded_bytes > MAX_FORWARDING_HEADER_BYTES {
            return Err(WebSecurityError::InvalidForwardedChain);
        }
        for item in value.split(',') {
            if addresses.len() >= MAX_FORWARDED_HOPS {
                return Err(WebSecurityError::InvalidForwardedChain);
            }
            let item = item.trim();
            if item.is_empty() {
                return Err(WebSecurityError::InvalidForwardedChain);
            }
            addresses.push(
                item.parse::<IpAddr>()
                    .map_err(|_| WebSecurityError::InvalidForwardedChain)?,
            );
        }
    }
    Ok(addresses)
}

pub fn security_cookie(
    headers: &HeaderMap,
    name: &str,
) -> Result<Option<Zeroizing<String>>, WebSecurityError> {
    let mut total_bytes = 0_usize;
    let mut pair_count = 0_usize;
    let mut matched: Option<Zeroizing<String>> = None;
    for value in headers.get_all(header::COOKIE) {
        let value = value
            .to_str()
            .map_err(|_| WebSecurityError::InvalidCookieHeader)?;
        total_bytes = total_bytes
            .checked_add(value.len())
            .ok_or(WebSecurityError::InvalidCookieHeader)?;
        if total_bytes > MAX_COOKIE_HEADER_BYTES {
            return Err(WebSecurityError::InvalidCookieHeader);
        }
        for encoded_pair in value.split(';') {
            pair_count += 1;
            if pair_count > MAX_COOKIE_PAIRS {
                return Err(WebSecurityError::InvalidCookieHeader);
            }
            let (cookie_name, cookie_value) = encoded_pair
                .trim()
                .split_once('=')
                .ok_or(WebSecurityError::InvalidCookieHeader)?;
            if cookie_name != name {
                continue;
            }
            if cookie_value.is_empty() || cookie_value.len() > MAX_SECURITY_COOKIE_BYTES {
                return Err(WebSecurityError::InvalidSecurityCookie);
            }
            if matched.is_some() {
                return Err(WebSecurityError::DuplicateSecurityCookie);
            }
            matched = Some(Zeroizing::new(cookie_value.to_owned()));
        }
    }
    Ok(matched)
}

pub fn csrf_header_and_cookie(
    headers: &HeaderMap,
) -> Result<(Zeroizing<String>, Zeroizing<String>), WebSecurityError> {
    let cookie =
        security_cookie(headers, CSRF_COOKIE_NAME)?.ok_or(WebSecurityError::CsrfMissing)?;
    let header_value = unique_header(headers, HeaderName::from_static(CSRF_HEADER_NAME))?
        .ok_or(WebSecurityError::CsrfMissing)?;
    if header_value.is_empty() || header_value.len() > MAX_SECURITY_COOKIE_BYTES {
        return Err(WebSecurityError::CsrfInvalid);
    }
    if !constant_time_bounded_string_equal(cookie.as_str(), header_value) {
        return Err(WebSecurityError::CsrfInvalid);
    }
    CsrfToken::parse_presented(cookie.as_str()).map_err(|_| WebSecurityError::CsrfInvalid)?;
    Ok((cookie, Zeroizing::new(header_value.to_owned())))
}

#[must_use]
pub fn session_set_cookie(value: &SessionToken, max_age_seconds: u64) -> String {
    format!(
        "{SESSION_COOKIE_NAME}={}; Path=/; Max-Age={max_age_seconds}; Secure; HttpOnly; SameSite=Lax",
        value.as_str()
    )
}

#[must_use]
pub fn csrf_set_cookie(value: &CsrfToken, max_age_seconds: u64) -> String {
    format!(
        "{CSRF_COOKIE_NAME}={}; Path=/; Max-Age={max_age_seconds}; Secure; SameSite=Lax",
        value.as_str()
    )
}

#[must_use]
pub fn clear_session_cookie() -> &'static str {
    "__Host-nodecontroll_session=; Path=/; Max-Age=0; Secure; HttpOnly; SameSite=Lax"
}

#[must_use]
pub fn clear_csrf_cookie() -> &'static str {
    "__Host-nodecontroll_csrf=; Path=/; Max-Age=0; Secure; SameSite=Lax"
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WebSecurityError {
    OriginMissing,
    OriginMismatch,
    HostMissing,
    HostMismatch,
    DuplicateSecurityHeader,
    InvalidSecurityHeader,
    InvalidForwardedChain,
    InvalidCookieHeader,
    InvalidSecurityCookie,
    DuplicateSecurityCookie,
    CsrfMissing,
    CsrfInvalid,
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, SocketAddr};

    use axum::http::{HeaderMap, HeaderValue, header};
    use nodecontroll_config::{PublicOrigin, TrustedProxyCidr};
    use nodecontroll_identity::{CsrfToken, SessionToken};

    use super::{
        CSRF_COOKIE_NAME, CSRF_HEADER_NAME, MAX_COOKIE_HEADER_BYTES, MAX_COOKIE_PAIRS,
        MAX_FORWARDED_HOPS, MAX_FORWARDING_HEADER_BYTES, MAX_SECURITY_COOKIE_BYTES,
        SESSION_COOKIE_NAME, WebSecurityError, WebSecurityPolicy, bounded_user_agent,
        clear_csrf_cookie, clear_session_cookie, csrf_header_and_cookie, csrf_set_cookie,
        security_cookie, session_set_cookie,
    };

    fn policy() -> WebSecurityPolicy {
        let origin = PublicOrigin::parse("https://panel.example.com");
        let proxy = TrustedProxyCidr::parse("10.0.0.0/8");
        assert!(origin.is_ok());
        assert!(proxy.is_ok());
        match (origin, proxy) {
            (Ok(origin), Ok(proxy)) => WebSecurityPolicy::new(origin, vec![proxy]),
            _ => unreachable!("checked above"),
        }
    }

    fn csrf_headers(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let cookie = HeaderValue::from_str(&format!("{CSRF_COOKIE_NAME}={token}"));
        let csrf = HeaderValue::from_str(token);
        assert!(cookie.is_ok());
        assert!(csrf.is_ok());
        if let (Ok(cookie), Ok(csrf)) = (cookie, csrf) {
            headers.insert(header::COOKIE, cookie);
            headers.insert(CSRF_HEADER_NAME, csrf);
        }
        headers
    }

    #[test]
    fn origin_and_host_must_match_the_canonical_public_origin() {
        let policy = policy();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ORIGIN,
            HeaderValue::from_static("https://panel.example.com"),
        );
        headers.insert(header::HOST, HeaderValue::from_static("panel.example.com"));
        assert_eq!(policy.validate_browser_origin(&headers), Ok(()));
        assert_eq!(policy.validate_request_host(&headers), Ok(()));
        headers.insert(
            header::ORIGIN,
            HeaderValue::from_static("https://evil.example"),
        );
        assert_eq!(
            policy.validate_browser_origin(&headers),
            Err(WebSecurityError::OriginMismatch)
        );
    }

    #[test]
    fn missing_and_duplicate_origin_or_host_are_rejected() {
        let policy = policy();
        let empty = HeaderMap::new();
        assert_eq!(
            policy.validate_browser_origin(&empty),
            Err(WebSecurityError::OriginMissing)
        );
        assert_eq!(
            policy.validate_request_host(&empty),
            Err(WebSecurityError::HostMissing)
        );

        let mut origin_only = HeaderMap::new();
        origin_only.insert(
            header::ORIGIN,
            HeaderValue::from_static("https://panel.example.com"),
        );
        assert_eq!(
            policy.validate_browser_origin(&origin_only),
            Err(WebSecurityError::HostMissing)
        );

        let mut host_only = HeaderMap::new();
        host_only.insert(header::HOST, HeaderValue::from_static("panel.example.com"));
        assert_eq!(
            policy.validate_browser_origin(&host_only),
            Err(WebSecurityError::OriginMissing)
        );
        host_only.insert(header::HOST, HeaderValue::from_static("evil.example"));
        assert_eq!(
            policy.validate_request_host(&host_only),
            Err(WebSecurityError::HostMismatch)
        );

        let mut duplicate_origin = HeaderMap::new();
        duplicate_origin.append(
            header::ORIGIN,
            HeaderValue::from_static("https://panel.example.com"),
        );
        duplicate_origin.append(
            header::ORIGIN,
            HeaderValue::from_static("https://panel.example.com"),
        );
        duplicate_origin.insert(header::HOST, HeaderValue::from_static("panel.example.com"));
        assert_eq!(
            policy.validate_browser_origin(&duplicate_origin),
            Err(WebSecurityError::DuplicateSecurityHeader)
        );

        let mut duplicate_host = HeaderMap::new();
        duplicate_host.insert(
            header::ORIGIN,
            HeaderValue::from_static("https://panel.example.com"),
        );
        duplicate_host.append(header::HOST, HeaderValue::from_static("panel.example.com"));
        duplicate_host.append(header::HOST, HeaderValue::from_static("panel.example.com"));
        assert_eq!(
            policy.validate_browser_origin(&duplicate_host),
            Err(WebSecurityError::DuplicateSecurityHeader)
        );
        assert_eq!(
            policy.validate_request_host(&duplicate_host),
            Err(WebSecurityError::DuplicateSecurityHeader)
        );
    }

    #[test]
    fn user_agent_is_unique_and_bounded() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static("NodeControll test"),
        );
        assert_eq!(
            bounded_user_agent(&headers).as_deref(),
            Ok("NodeControll test")
        );
        let overlong = HeaderValue::from_str(&"a".repeat(513));
        assert!(overlong.is_ok());
        if let Ok(overlong) = overlong {
            headers.insert(header::USER_AGENT, overlong);
        }
        assert_eq!(
            bounded_user_agent(&headers),
            Err(WebSecurityError::InvalidSecurityHeader)
        );
    }

    #[test]
    fn untrusted_forwarding_headers_are_ignored_and_trusted_chains_are_walked() {
        let policy = policy();
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("203.0.113.99, 198.51.100.4, 10.2.0.8"),
        );
        let untrusted_peer = SocketAddr::from(([203, 0, 113, 9], 443));
        let direct = policy.resolve_client_network(untrusted_peer, &headers);
        assert!(matches!(direct, Ok(ref network) if network.address() == untrusted_peer.ip()));

        let trusted_peer = SocketAddr::from(([10, 1, 0, 2], 443));
        let forwarded = policy.resolve_client_network(trusted_peer, &headers);
        assert!(matches!(
            forwarded,
            Ok(ref network) if network.address() == "198.51.100.4".parse::<IpAddr>().unwrap_or(trusted_peer.ip())
        ));

        assert_eq!(
            policy.resolve_client_network(trusted_peer, &HeaderMap::new()),
            Err(WebSecurityError::InvalidForwardedChain)
        );
    }

    #[test]
    fn forwarded_hop_and_encoded_byte_limits_are_enforced() {
        let policy = policy();
        let trusted_peer = SocketAddr::from(([10, 1, 0, 2], 443));

        let at_hop_limit = (1..=MAX_FORWARDED_HOPS)
            .map(|last_octet| format!("10.0.0.{last_octet}"))
            .collect::<Vec<_>>()
            .join(", ");
        let at_hop_limit = HeaderValue::from_str(&at_hop_limit);
        assert!(at_hop_limit.is_ok());
        let mut headers = HeaderMap::new();
        if let Ok(at_hop_limit) = at_hop_limit {
            headers.insert("x-forwarded-for", at_hop_limit);
        }
        assert!(
            policy
                .resolve_client_network(trusted_peer, &headers)
                .is_ok()
        );

        let over_hop_limit = (1..=MAX_FORWARDED_HOPS + 1)
            .map(|last_octet| format!("10.0.0.{last_octet}"))
            .collect::<Vec<_>>()
            .join(", ");
        let over_hop_limit = HeaderValue::from_str(&over_hop_limit);
        assert!(over_hop_limit.is_ok());
        headers.clear();
        if let Ok(over_hop_limit) = over_hop_limit {
            headers.insert("x-forwarded-for", over_hop_limit);
        }
        assert_eq!(
            policy.resolve_client_network(trusted_peer, &headers),
            Err(WebSecurityError::InvalidForwardedChain)
        );

        let client = "198.51.100.4";
        assert!(client.len() < MAX_FORWARDING_HEADER_BYTES);
        let at_byte_limit = format!(
            "{client}{}",
            " ".repeat(MAX_FORWARDING_HEADER_BYTES - client.len())
        );
        assert_eq!(at_byte_limit.len(), MAX_FORWARDING_HEADER_BYTES);
        let at_byte_limit = HeaderValue::from_str(&at_byte_limit);
        assert!(at_byte_limit.is_ok());
        headers.clear();
        if let Ok(at_byte_limit) = at_byte_limit {
            headers.insert("x-forwarded-for", at_byte_limit);
        }
        let resolved = policy.resolve_client_network(trusted_peer, &headers);
        let expected = client.parse::<IpAddr>();
        assert!(matches!(
            (resolved, expected),
            (Ok(ref network), Ok(expected)) if network.address() == expected
        ));

        let over_byte_limit = format!(
            "{client}{}",
            " ".repeat(MAX_FORWARDING_HEADER_BYTES + 1 - client.len())
        );
        assert_eq!(over_byte_limit.len(), MAX_FORWARDING_HEADER_BYTES + 1);
        let over_byte_limit = HeaderValue::from_str(&over_byte_limit);
        assert!(over_byte_limit.is_ok());
        headers.clear();
        if let Ok(over_byte_limit) = over_byte_limit {
            headers.insert("x-forwarded-for", over_byte_limit);
        }
        assert_eq!(
            policy.resolve_client_network(trusted_peer, &headers),
            Err(WebSecurityError::InvalidForwardedChain)
        );
    }

    #[test]
    fn duplicate_or_mismatched_csrf_material_is_rejected() {
        let token = format!("ncc1_{}", "a".repeat(64));
        let mut headers = csrf_headers(&token);
        assert!(csrf_header_and_cookie(&headers).is_ok());
        headers.insert(CSRF_HEADER_NAME, HeaderValue::from_static("ncc1_wrong"));
        assert_eq!(
            csrf_header_and_cookie(&headers),
            Err(WebSecurityError::CsrfInvalid)
        );

        let duplicate = format!("{SESSION_COOKIE_NAME}=one; {SESSION_COOKIE_NAME}=two");
        let duplicate = HeaderValue::from_str(&duplicate);
        assert!(duplicate.is_ok());
        if let Ok(duplicate) = duplicate {
            headers.insert(header::COOKIE, duplicate);
        }
        assert_eq!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Err(WebSecurityError::DuplicateSecurityCookie)
        );
    }

    #[test]
    fn csrf_material_requires_strict_versioned_lowercase_hex_format() {
        let valid = format!("ncc1_{}", "a".repeat(64));
        assert!(CsrfToken::parse_presented(&valid).is_ok());
        assert!(csrf_header_and_cookie(&csrf_headers(&valid)).is_ok());

        let invalid = [
            format!("ncc2_{}", "a".repeat(64)),
            format!("ncc1_{}", "A".repeat(64)),
            format!("ncc1_{}", "a".repeat(63)),
            format!("ncc1_{}g", "a".repeat(63)),
            format!("ncs1_{}", "a".repeat(64)),
        ];
        for token in invalid {
            assert!(CsrfToken::parse_presented(&token).is_err());
            assert_eq!(
                csrf_header_and_cookie(&csrf_headers(&token)),
                Err(WebSecurityError::CsrfInvalid)
            );
        }
    }

    #[test]
    fn security_cookie_scans_all_cookie_headers_and_rejects_cross_header_duplicates() {
        let token = format!("ncs1_{}", "a".repeat(64));
        let encoded_session = HeaderValue::from_str(&format!("{SESSION_COOKIE_NAME}={token}"));
        assert!(encoded_session.is_ok());

        let mut headers = HeaderMap::new();
        headers.append(header::COOKIE, HeaderValue::from_static("theme=dark"));
        if let Ok(encoded_session) = encoded_session {
            headers.append(header::COOKIE, encoded_session);
        }
        assert!(matches!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Ok(Some(ref value)) if value.as_str() == token
        ));

        let duplicate = HeaderValue::from_str(&format!("{SESSION_COOKIE_NAME}={token}"));
        assert!(duplicate.is_ok());
        if let Ok(duplicate) = duplicate {
            headers.append(header::COOKIE, duplicate);
        }
        assert_eq!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Err(WebSecurityError::DuplicateSecurityCookie)
        );
    }

    #[test]
    fn cookie_pair_and_encoded_byte_limits_are_enforced_at_the_boundary() {
        let token = format!("ncs1_{}", "b".repeat(64));
        let mut pairs = (0..MAX_COOKIE_PAIRS - 1)
            .map(|index| format!("cookie{index}=value"))
            .collect::<Vec<_>>();
        pairs.push(format!("{SESSION_COOKIE_NAME}={token}"));
        let at_pair_limit = HeaderValue::from_str(&pairs.join("; "));
        assert!(at_pair_limit.is_ok());
        let mut headers = HeaderMap::new();
        if let Ok(at_pair_limit) = at_pair_limit {
            headers.insert(header::COOKIE, at_pair_limit);
        }
        assert!(matches!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Ok(Some(ref value)) if value.as_str() == token
        ));

        pairs.push("one_too_many=value".to_owned());
        let over_pair_limit = HeaderValue::from_str(&pairs.join("; "));
        assert!(over_pair_limit.is_ok());
        headers.clear();
        if let Ok(over_pair_limit) = over_pair_limit {
            headers.insert(header::COOKIE, over_pair_limit);
        }
        assert_eq!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Err(WebSecurityError::InvalidCookieHeader)
        );

        let cookie_prefix = "unrelated=";
        assert!(cookie_prefix.len() < MAX_COOKIE_HEADER_BYTES);
        let at_byte_limit = format!(
            "{cookie_prefix}{}",
            "a".repeat(MAX_COOKIE_HEADER_BYTES - cookie_prefix.len())
        );
        assert_eq!(at_byte_limit.len(), MAX_COOKIE_HEADER_BYTES);
        let at_byte_limit = HeaderValue::from_str(&at_byte_limit);
        assert!(at_byte_limit.is_ok());
        headers.clear();
        if let Ok(at_byte_limit) = at_byte_limit {
            headers.append(header::COOKIE, at_byte_limit);
        }
        assert!(matches!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Ok(None)
        ));

        headers.append(header::COOKIE, HeaderValue::from_static("extra=value"));
        assert_eq!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Err(WebSecurityError::InvalidCookieHeader)
        );
    }

    #[test]
    fn security_cookie_value_size_is_enforced_at_the_boundary() {
        let at_limit = "a".repeat(MAX_SECURITY_COOKIE_BYTES);
        let encoded = HeaderValue::from_str(&format!("{SESSION_COOKIE_NAME}={at_limit}"));
        assert!(encoded.is_ok());
        let mut headers = HeaderMap::new();
        if let Ok(encoded) = encoded {
            headers.insert(header::COOKIE, encoded);
        }
        assert!(matches!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Ok(Some(ref value)) if value.len() == MAX_SECURITY_COOKIE_BYTES
        ));

        let over_limit = "a".repeat(MAX_SECURITY_COOKIE_BYTES + 1);
        let encoded = HeaderValue::from_str(&format!("{SESSION_COOKIE_NAME}={over_limit}"));
        assert!(encoded.is_ok());
        headers.clear();
        if let Ok(encoded) = encoded {
            headers.insert(header::COOKIE, encoded);
        }
        assert_eq!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Err(WebSecurityError::InvalidSecurityCookie)
        );

        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("__Host-nodecontroll_session="),
        );
        assert_eq!(
            security_cookie(&headers, SESSION_COOKIE_NAME),
            Err(WebSecurityError::InvalidSecurityCookie)
        );
    }

    #[test]
    fn cookie_contract_is_host_only_secure_and_session_is_http_only() {
        let session_token = SessionToken::parse_presented(&format!("ncs1_{}", "a".repeat(64)));
        let csrf_token = CsrfToken::parse_presented(&format!("ncc1_{}", "b".repeat(64)));
        assert!(session_token.is_ok());
        assert!(csrf_token.is_ok());
        let (Ok(session_token), Ok(csrf_token)) = (session_token, csrf_token) else {
            return;
        };
        let session = session_set_cookie(&session_token, 3_600);
        let csrf = csrf_set_cookie(&csrf_token, 3_600);
        assert!(session.starts_with("__Host-nodecontroll_session="));
        assert!(session.contains("; Path=/;"));
        assert!(session.contains("; Secure;"));
        assert!(session.contains("; HttpOnly;"));
        assert!(session.ends_with("SameSite=Lax"));
        assert!(!session.contains("Domain="));
        assert!(csrf.starts_with("__Host-nodecontroll_csrf="));
        assert!(csrf.contains("; Secure;"));
        assert!(!csrf.contains("HttpOnly"));
        assert!(!csrf.contains("Domain="));
        assert!(clear_session_cookie().contains("Max-Age=0"));
        assert!(clear_csrf_cookie().contains("Max-Age=0"));
    }
}
