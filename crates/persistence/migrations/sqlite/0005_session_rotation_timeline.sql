CREATE TABLE auth_sessions_new (
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
    auth_level TEXT NOT NULL CHECK (
        auth_level IN ('password','mfa','phishing_resistant','recovery')
    ),
    status TEXT NOT NULL CHECK (status IN ('active','revoked')),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    authenticated_at_ms INTEGER NOT NULL CHECK (authenticated_at_ms >= 0),
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
            'user_revoked','administrator','rotation','expired','security_policy'
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

INSERT INTO auth_sessions_new (
    id, user_id, token_key_version, token_hmac, csrf_key_version, csrf_hmac,
    auth_revision, auth_level, status, created_at_ms, authenticated_at_ms,
    recent_auth_at_ms, last_seen_at_ms, idle_expires_at_ms, absolute_expires_at_ms,
    ip_prefix_key_version, ip_prefix_hmac, user_agent_hash, revoked_at_ms,
    revoked_reason, revision
)
SELECT
    id, user_id, token_key_version, token_hmac, csrf_key_version, csrf_hmac,
    auth_revision,
    CASE auth_level
        WHEN 'webauthn' THEN 'phishing_resistant'
        ELSE auth_level
    END,
    status, created_at_ms, authenticated_at_ms,
    recent_auth_at_ms, last_seen_at_ms, idle_expires_at_ms, absolute_expires_at_ms,
    ip_prefix_key_version, ip_prefix_hmac, user_agent_hash, revoked_at_ms,
    revoked_reason, revision
FROM auth_sessions;

DROP TABLE auth_sessions;

ALTER TABLE auth_sessions_new RENAME TO auth_sessions;

CREATE INDEX auth_sessions_user_created_idx
    ON auth_sessions(user_id, created_at_ms DESC, id);

CREATE INDEX auth_sessions_active_user_idx
    ON auth_sessions(user_id, absolute_expires_at_ms, idle_expires_at_ms)
    WHERE status = 'active';
