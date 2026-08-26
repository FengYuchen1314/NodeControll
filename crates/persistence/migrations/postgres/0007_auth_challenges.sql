CREATE TABLE auth_challenges (
    id UUID PRIMARY KEY,
    token_key_version INTEGER NOT NULL CHECK (token_key_version > 0),
    token_hmac BYTEA NOT NULL CHECK (octet_length(token_hmac) = 32),
    purpose TEXT NOT NULL CHECK (purpose IN (
        'login','reauthenticate','sensitive_action','credential_enrollment'
    )),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_id UUID REFERENCES auth_sessions(id) ON DELETE RESTRICT,
    auth_revision BIGINT NOT NULL CHECK (auth_revision >= 0),
    status TEXT NOT NULL CHECK (status IN (
        'pending','verification_pending','rotation_pending','consumed',
        'exhausted','expired','invalidated'
    )),
    rotation_state TEXT NOT NULL CHECK (rotation_state IN (
        'not_required','required','pending','completed'
    )),
    attempts_used INTEGER NOT NULL DEFAULT 0 CHECK (
        attempts_used BETWEEN 0 AND max_attempts
    ),
    max_attempts INTEGER NOT NULL CHECK (max_attempts > 0),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    expires_at_ms BIGINT NOT NULL CHECK (expires_at_ms > created_at_ms),
    attempt_claim_id UUID,
    attempted_method TEXT CHECK (attempted_method IS NULL OR attempted_method IN (
        'password','totp','webauthn','recovery_code'
    )),
    attempt_started_at_ms BIGINT CHECK (
        attempt_started_at_ms IS NULL OR (
            attempt_started_at_ms >= created_at_ms AND attempt_started_at_ms < expires_at_ms
        )
    ),
    attempt_expires_at_ms BIGINT CHECK (
        attempt_expires_at_ms IS NULL OR (
            attempt_expires_at_ms > attempt_started_at_ms
            AND attempt_expires_at_ms <= expires_at_ms
        )
    ),
    verified_method TEXT CHECK (verified_method IS NULL OR verified_method IN (
        'password','totp','webauthn','recovery_code'
    )),
    achieved_assurance TEXT CHECK (achieved_assurance IS NULL OR achieved_assurance IN (
        'password','mfa','phishing_resistant','recovery'
    )),
    consumed_at_ms BIGINT CHECK (
        consumed_at_ms IS NULL OR (
            consumed_at_ms >= created_at_ms AND consumed_at_ms < expires_at_ms
        )
    ),
    context_key_version INTEGER CHECK (
        context_key_version IS NULL OR context_key_version > 0
    ),
    client_network_hmac BYTEA CHECK (
        client_network_hmac IS NULL OR octet_length(client_network_hmac) = 32
    ),
    user_agent_hash BYTEA CHECK (
        user_agent_hash IS NULL OR octet_length(user_agent_hash) = 32
    ),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    updated_at_ms BIGINT NOT NULL CHECK (updated_at_ms >= created_at_ms),
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
    challenge_id UUID NOT NULL REFERENCES auth_challenges(id) ON DELETE CASCADE,
    method TEXT NOT NULL CHECK (method IN (
        'password','totp','webauthn','recovery_code'
    )),
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
