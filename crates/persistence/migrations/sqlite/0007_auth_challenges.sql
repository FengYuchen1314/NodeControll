CREATE TABLE auth_challenges (
    id TEXT PRIMARY KEY CHECK (
        typeof(id) = 'text' AND length(id) = 36
        AND substr(id, 9, 1) = '-' AND substr(id, 14, 1) = '-'
        AND substr(id, 19, 1) = '-' AND substr(id, 24, 1) = '-'
        AND length(replace(id, '-', '')) = 32
        AND replace(id, '-', '') NOT GLOB '*[^0-9a-f]*'
    ),
    token_key_version INTEGER NOT NULL CHECK (
        typeof(token_key_version) = 'integer'
        AND token_key_version BETWEEN 1 AND 2147483647
    ),
    token_hmac BLOB NOT NULL CHECK (
        typeof(token_hmac) = 'blob' AND length(token_hmac) = 32
    ),
    purpose TEXT NOT NULL CHECK (
        typeof(purpose) = 'text'
        AND purpose IN ('login','reauthenticate','sensitive_action','credential_enrollment')
    ),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE CHECK (
        typeof(user_id) = 'text' AND length(user_id) = 36
        AND substr(user_id, 9, 1) = '-' AND substr(user_id, 14, 1) = '-'
        AND substr(user_id, 19, 1) = '-' AND substr(user_id, 24, 1) = '-'
        AND length(replace(user_id, '-', '')) = 32
        AND replace(user_id, '-', '') NOT GLOB '*[^0-9a-f]*'
    ),
    session_id TEXT REFERENCES auth_sessions(id) ON DELETE RESTRICT CHECK (
        session_id IS NULL OR (
            typeof(session_id) = 'text' AND length(session_id) = 36
            AND substr(session_id, 9, 1) = '-' AND substr(session_id, 14, 1) = '-'
            AND substr(session_id, 19, 1) = '-' AND substr(session_id, 24, 1) = '-'
            AND length(replace(session_id, '-', '')) = 32
            AND replace(session_id, '-', '') NOT GLOB '*[^0-9a-f]*'
        )
    ),
    auth_revision INTEGER NOT NULL CHECK (
        typeof(auth_revision) = 'integer' AND auth_revision >= 0
    ),
    status TEXT NOT NULL CHECK (
        typeof(status) = 'text'
        AND status IN (
            'pending','verification_pending','rotation_pending','consumed',
            'exhausted','expired','invalidated'
        )
    ),
    rotation_state TEXT NOT NULL CHECK (
        typeof(rotation_state) = 'text'
        AND rotation_state IN ('not_required','required','pending','completed')
    ),
    attempts_used INTEGER NOT NULL DEFAULT 0 CHECK (
        typeof(attempts_used) = 'integer'
        AND attempts_used BETWEEN 0 AND max_attempts
    ),
    max_attempts INTEGER NOT NULL CHECK (
        typeof(max_attempts) = 'integer'
        AND max_attempts BETWEEN 1 AND 2147483647
    ),
    created_at_ms INTEGER NOT NULL CHECK (
        typeof(created_at_ms) = 'integer' AND created_at_ms >= 0
    ),
    expires_at_ms INTEGER NOT NULL CHECK (
        typeof(expires_at_ms) = 'integer' AND expires_at_ms > created_at_ms
    ),
    attempt_claim_id TEXT CHECK (
        attempt_claim_id IS NULL OR (
            typeof(attempt_claim_id) = 'text' AND length(attempt_claim_id) = 36
            AND substr(attempt_claim_id, 9, 1) = '-' AND substr(attempt_claim_id, 14, 1) = '-'
            AND substr(attempt_claim_id, 19, 1) = '-' AND substr(attempt_claim_id, 24, 1) = '-'
            AND length(replace(attempt_claim_id, '-', '')) = 32
            AND replace(attempt_claim_id, '-', '') NOT GLOB '*[^0-9a-f]*'
        )
    ),
    attempted_method TEXT CHECK (
        attempted_method IS NULL OR (
            typeof(attempted_method) = 'text'
            AND attempted_method IN ('password','totp','webauthn','recovery_code')
        )
    ),
    attempt_started_at_ms INTEGER CHECK (
        attempt_started_at_ms IS NULL OR (
            typeof(attempt_started_at_ms) = 'integer'
            AND attempt_started_at_ms >= created_at_ms
            AND attempt_started_at_ms < expires_at_ms
        )
    ),
    attempt_expires_at_ms INTEGER CHECK (
        attempt_expires_at_ms IS NULL OR (
            typeof(attempt_expires_at_ms) = 'integer'
            AND attempt_expires_at_ms > attempt_started_at_ms
            AND attempt_expires_at_ms <= expires_at_ms
        )
    ),
    verified_method TEXT CHECK (
        verified_method IS NULL OR (
            typeof(verified_method) = 'text'
            AND verified_method IN ('password','totp','webauthn','recovery_code')
        )
    ),
    achieved_assurance TEXT CHECK (
        achieved_assurance IS NULL OR (
            typeof(achieved_assurance) = 'text'
            AND achieved_assurance IN ('password','mfa','phishing_resistant','recovery')
        )
    ),
    consumed_at_ms INTEGER CHECK (
        consumed_at_ms IS NULL OR (
            typeof(consumed_at_ms) = 'integer'
            AND consumed_at_ms >= created_at_ms AND consumed_at_ms < expires_at_ms
        )
    ),
    context_key_version INTEGER CHECK (
        context_key_version IS NULL OR (
            typeof(context_key_version) = 'integer'
            AND context_key_version BETWEEN 1 AND 2147483647
        )
    ),
    client_network_hmac BLOB CHECK (
        client_network_hmac IS NULL OR (
            typeof(client_network_hmac) = 'blob' AND length(client_network_hmac) = 32
        )
    ),
    user_agent_hash BLOB CHECK (
        user_agent_hash IS NULL OR (
            typeof(user_agent_hash) = 'blob' AND length(user_agent_hash) = 32
        )
    ),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (
        typeof(revision) = 'integer' AND revision >= 0
    ),
    updated_at_ms INTEGER NOT NULL CHECK (
        typeof(updated_at_ms) = 'integer' AND updated_at_ms >= created_at_ms
    ),
    UNIQUE (token_key_version, token_hmac),
    CHECK (
        (context_key_version IS NULL AND client_network_hmac IS NULL AND user_agent_hash IS NULL)
        OR
        (context_key_version IS NOT NULL AND client_network_hmac IS NOT NULL AND user_agent_hash IS NOT NULL)
    ),
    CHECK (
        (verified_method IS NULL AND achieved_assurance IS NULL)
        OR (verified_method = 'password' AND achieved_assurance = 'password')
        OR (verified_method = 'totp' AND achieved_assurance = 'mfa')
        OR (verified_method = 'webauthn' AND achieved_assurance IN ('mfa','phishing_resistant'))
        OR (verified_method = 'recovery_code' AND achieved_assurance = 'recovery')
    ),
    CHECK (
        (status = 'pending'
            AND attempts_used < max_attempts
            AND attempt_claim_id IS NULL AND attempted_method IS NULL
            AND attempt_started_at_ms IS NULL AND attempt_expires_at_ms IS NULL
            AND verified_method IS NULL AND achieved_assurance IS NULL
            AND consumed_at_ms IS NULL
            AND rotation_state IN ('not_required','required'))
        OR
        (status = 'verification_pending'
            AND attempts_used BETWEEN 1 AND max_attempts
            AND attempt_claim_id IS NOT NULL AND attempted_method IS NOT NULL
            AND attempt_started_at_ms IS NOT NULL AND attempt_expires_at_ms IS NOT NULL
            AND verified_method IS NULL AND achieved_assurance IS NULL
            AND consumed_at_ms IS NULL
            AND rotation_state IN ('not_required','required'))
        OR
        (status = 'exhausted'
            AND attempts_used = max_attempts
            AND attempt_claim_id IS NULL AND attempted_method IS NULL
            AND attempt_started_at_ms IS NULL AND attempt_expires_at_ms IS NULL
            AND verified_method IS NULL AND achieved_assurance IS NULL
            AND consumed_at_ms IS NULL
            AND rotation_state IN ('not_required','required'))
        OR
        (status = 'rotation_pending'
            AND attempts_used BETWEEN 1 AND max_attempts
            AND verified_method IS NOT NULL AND achieved_assurance IS NOT NULL
            AND consumed_at_ms IS NULL AND rotation_state = 'pending'
            AND (
                (attempt_claim_id IS NULL AND attempted_method IS NULL
                    AND attempt_started_at_ms IS NULL AND attempt_expires_at_ms IS NULL)
                OR
                (attempt_claim_id IS NOT NULL AND attempted_method = verified_method
                    AND attempt_started_at_ms IS NOT NULL AND attempt_expires_at_ms IS NOT NULL)
            ))
        OR
        (status = 'consumed'
            AND attempts_used BETWEEN 1 AND max_attempts
            AND attempt_claim_id IS NULL AND attempted_method IS NULL
            AND attempt_started_at_ms IS NULL AND attempt_expires_at_ms IS NULL
            AND verified_method IS NOT NULL AND achieved_assurance IS NOT NULL
            AND consumed_at_ms IS NOT NULL
            AND rotation_state IN ('not_required','completed'))
        OR
        (status IN ('expired','invalidated')
            AND attempt_claim_id IS NULL AND attempted_method IS NULL
            AND attempt_started_at_ms IS NULL AND attempt_expires_at_ms IS NULL
            AND consumed_at_ms IS NULL AND rotation_state <> 'completed'
            AND (
                (verified_method IS NULL AND achieved_assurance IS NULL
                    AND rotation_state IN ('not_required','required'))
                OR
                (verified_method IS NOT NULL AND achieved_assurance IS NOT NULL
                    AND rotation_state = 'pending')
            ))
    )
);

CREATE TABLE auth_challenge_methods (
    challenge_id TEXT NOT NULL REFERENCES auth_challenges(id) ON DELETE CASCADE CHECK (
        typeof(challenge_id) = 'text' AND length(challenge_id) = 36
        AND substr(challenge_id, 9, 1) = '-' AND substr(challenge_id, 14, 1) = '-'
        AND substr(challenge_id, 19, 1) = '-' AND substr(challenge_id, 24, 1) = '-'
        AND length(replace(challenge_id, '-', '')) = 32
        AND replace(challenge_id, '-', '') NOT GLOB '*[^0-9a-f]*'
    ),
    method TEXT NOT NULL CHECK (
        typeof(method) = 'text'
        AND method IN ('password','totp','webauthn','recovery_code')
    ),
    PRIMARY KEY (challenge_id, method)
);

-- Exhaustion keeps this durable limiter slot until expiry, so minting a new bearer cannot reset
-- the guess budget. Expired/invalidated/consumed challenges release it.
CREATE UNIQUE INDEX auth_challenges_user_purpose_open_uq
    ON auth_challenges(user_id, purpose)
    WHERE status IN ('pending','verification_pending','rotation_pending','exhausted');

CREATE INDEX auth_challenges_user_status_idx
    ON auth_challenges(user_id, status, expires_at_ms, id);

CREATE INDEX auth_challenges_expiry_idx
    ON auth_challenges(status, expires_at_ms, attempt_expires_at_ms, id)
    WHERE status IN ('pending','verification_pending','rotation_pending','exhausted');

CREATE INDEX auth_challenges_session_idx
    ON auth_challenges(session_id, status, id)
    WHERE session_id IS NOT NULL;
