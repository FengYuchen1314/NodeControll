CREATE TABLE login_security_events_new (
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
        'auth_revision_changed','reauthentication_succeeded','password_changed'
    )),
    digest_key_version INTEGER NOT NULL CHECK (digest_key_version > 0),
    account_hmac BLOB CHECK (account_hmac IS NULL OR length(account_hmac) = 32),
    ip_prefix_hmac BLOB CHECK (ip_prefix_hmac IS NULL OR length(ip_prefix_hmac) = 32),
    user_agent_hash BLOB CHECK (user_agent_hash IS NULL OR length(user_agent_hash) = 32)
);

INSERT INTO login_security_events_new (
    id, occurred_at_ms, request_id, reason, digest_key_version,
    account_hmac, ip_prefix_hmac, user_agent_hash
)
SELECT
    id, occurred_at_ms, request_id, reason, digest_key_version,
    account_hmac, ip_prefix_hmac, user_agent_hash
FROM login_security_events;

DROP TABLE login_security_events;

ALTER TABLE login_security_events_new RENAME TO login_security_events;

CREATE INDEX login_security_events_time_idx
    ON login_security_events(occurred_at_ms DESC, id);

CREATE INDEX login_security_events_account_idx
    ON login_security_events(account_hmac, occurred_at_ms DESC)
    WHERE account_hmac IS NOT NULL;
