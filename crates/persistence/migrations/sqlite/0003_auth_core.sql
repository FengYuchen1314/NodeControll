CREATE TABLE user_auth_state (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE CHECK (
        length(user_id) = 36
        AND user_id GLOB '????????-????-????-????-????????????'
        AND user_id NOT GLOB '*[^0-9A-Fa-f-]*'
    ),
    auth_revision INTEGER NOT NULL DEFAULT 0 CHECK (auth_revision >= 0),
    password_changed_at_ms INTEGER NOT NULL CHECK (password_changed_at_ms >= 0),
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms >= password_changed_at_ms)
);

INSERT INTO user_auth_state (
    user_id, auth_revision, password_changed_at_ms, updated_at_ms
)
SELECT id, 0, created_at_ms, created_at_ms
FROM users;

CREATE TABLE auth_sessions (
    id TEXT PRIMARY KEY CHECK (
        length(id) = 36
        AND id GLOB '????????-????-????-????-????????????'
        AND id NOT GLOB '*[^0-9A-Fa-f-]*'
    ),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE RESTRICT CHECK (
        length(user_id) = 36
        AND user_id GLOB '????????-????-????-????-????????????'
        AND user_id NOT GLOB '*[^0-9A-Fa-f-]*'
    ),
    token_key_version INTEGER NOT NULL CHECK (token_key_version > 0),
    token_hmac BLOB NOT NULL CHECK (length(token_hmac) = 32),
    csrf_key_version INTEGER NOT NULL CHECK (csrf_key_version > 0),
    csrf_hmac BLOB NOT NULL CHECK (length(csrf_hmac) = 32),
    auth_revision INTEGER NOT NULL CHECK (auth_revision >= 0),
    auth_level TEXT NOT NULL CHECK (auth_level IN ('password','mfa','webauthn','recovery')),
    status TEXT NOT NULL CHECK (status IN ('active','revoked')),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    authenticated_at_ms INTEGER NOT NULL CHECK (authenticated_at_ms >= created_at_ms),
    recent_auth_at_ms INTEGER NOT NULL CHECK (recent_auth_at_ms >= authenticated_at_ms),
    last_seen_at_ms INTEGER NOT NULL CHECK (
        last_seen_at_ms >= created_at_ms
        AND last_seen_at_ms >= recent_auth_at_ms
    ),
    idle_expires_at_ms INTEGER NOT NULL CHECK (
        idle_expires_at_ms > last_seen_at_ms
        AND idle_expires_at_ms <= absolute_expires_at_ms
    ),
    absolute_expires_at_ms INTEGER NOT NULL CHECK (absolute_expires_at_ms > created_at_ms),
    ip_prefix_key_version INTEGER CHECK (
        (ip_prefix_key_version IS NULL AND ip_prefix_hmac IS NULL)
        OR
        (ip_prefix_key_version > 0 AND ip_prefix_hmac IS NOT NULL)
    ),
    ip_prefix_hmac BLOB CHECK (ip_prefix_hmac IS NULL OR length(ip_prefix_hmac) = 32),
    user_agent_hash BLOB CHECK (user_agent_hash IS NULL OR length(user_agent_hash) = 32),
    revoked_at_ms INTEGER CHECK (revoked_at_ms IS NULL OR revoked_at_ms >= created_at_ms),
    revoked_reason TEXT CHECK (
        revoked_reason IS NULL OR revoked_reason IN (
            'logout','logout_all','password_changed','user_disabled',
            'administrator','rotation','expired','security_policy'
        )
    ),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    UNIQUE (token_key_version, token_hmac),
    UNIQUE (csrf_key_version, csrf_hmac),
    CHECK (
        (status = 'active' AND revoked_at_ms IS NULL AND revoked_reason IS NULL)
        OR
        (status = 'revoked' AND revoked_at_ms IS NOT NULL AND revoked_reason IS NOT NULL)
    )
);

CREATE INDEX auth_sessions_user_created_idx
    ON auth_sessions(user_id, created_at_ms DESC, id);

CREATE INDEX auth_sessions_active_user_idx
    ON auth_sessions(user_id, absolute_expires_at_ms, idle_expires_at_ms)
    WHERE status = 'active';

CREATE TABLE login_rate_buckets (
    scope TEXT NOT NULL CHECK (scope IN ('account','ip','global')),
    key_version INTEGER NOT NULL CHECK (key_version > 0),
    bucket_hmac BLOB NOT NULL CHECK (length(bucket_hmac) = 32),
    window_started_at_ms INTEGER NOT NULL CHECK (window_started_at_ms >= 0),
    window_expires_at_ms INTEGER NOT NULL CHECK (window_expires_at_ms > window_started_at_ms),
    attempt_count INTEGER NOT NULL CHECK (attempt_count BETWEEN 1 AND 2147483647),
    blocked_until_ms INTEGER CHECK (
        blocked_until_ms IS NULL OR blocked_until_ms >= window_started_at_ms
    ),
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms >= window_started_at_ms),
    PRIMARY KEY (scope, key_version, bucket_hmac)
);

CREATE INDEX login_rate_buckets_expiry_idx
    ON login_rate_buckets(window_expires_at_ms, blocked_until_ms);

CREATE TABLE login_security_events (
    id TEXT PRIMARY KEY CHECK (
        length(id) = 36
        AND id GLOB '????????-????-????-????-????????????'
        AND id NOT GLOB '*[^0-9A-Fa-f-]*'
    ),
    occurred_at_ms INTEGER NOT NULL CHECK (occurred_at_ms >= 0),
    request_id TEXT NOT NULL CHECK (length(request_id) BETWEEN 1 AND 128),
    reason TEXT NOT NULL CHECK (reason IN (
        'login_succeeded','invalid_credentials','rate_limited','account_inactive',
        'session_expired','session_revoked','csrf_mismatch','logout','logout_all',
        'auth_revision_changed'
    )),
    digest_key_version INTEGER NOT NULL CHECK (digest_key_version > 0),
    account_hmac BLOB CHECK (account_hmac IS NULL OR length(account_hmac) = 32),
    ip_prefix_hmac BLOB CHECK (ip_prefix_hmac IS NULL OR length(ip_prefix_hmac) = 32),
    user_agent_hash BLOB CHECK (user_agent_hash IS NULL OR length(user_agent_hash) = 32)
);

CREATE INDEX login_security_events_time_idx
    ON login_security_events(occurred_at_ms DESC, id);

CREATE INDEX login_security_events_account_idx
    ON login_security_events(account_hmac, occurred_at_ms DESC)
    WHERE account_hmac IS NOT NULL;
