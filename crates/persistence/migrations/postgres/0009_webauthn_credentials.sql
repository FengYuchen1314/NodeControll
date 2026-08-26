-- WP02-C5: encrypted, typed WebAuthn ceremony state and multi-passkey lifecycle.
-- Raw attestation objects, clientDataJSON and authenticatorData are deliberately absent.
CREATE TABLE webauthn_credentials (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    credential_id BYTEA NOT NULL UNIQUE CHECK (octet_length(credential_id) BETWEEN 16 AND 1023),
    user_handle BYTEA NOT NULL CHECK (octet_length(user_handle) = 16),
    aaguid UUID,
    user_verified BOOLEAN NOT NULL CHECK (user_verified),
    backup_eligible BOOLEAN NOT NULL,
    backup_state BOOLEAN NOT NULL CHECK (NOT backup_state OR backup_eligible),
    sign_counter BIGINT NOT NULL CHECK (sign_counter BETWEEN 0 AND 4294967295),
    nickname TEXT NOT NULL CHECK (
        char_length(btrim(nickname)) BETWEEN 1 AND 80 AND nickname = btrim(nickname)
    ),
    status TEXT NOT NULL CHECK (status IN ('active','revoked','clone_suspected')),
    material_schema_version INTEGER NOT NULL DEFAULT 1 CHECK (material_schema_version = 1),
    material_key_version INTEGER NOT NULL CHECK (material_key_version > 0),
    material_nonce BYTEA NOT NULL CHECK (octet_length(material_nonce) = 24),
    material_ciphertext BYTEA NOT NULL CHECK (
        octet_length(material_ciphertext) BETWEEN 17 AND 262144
    ),
    material_aad_hash BYTEA NOT NULL CHECK (octet_length(material_aad_hash) = 32),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    last_used_at_ms BIGINT CHECK (
        last_used_at_ms IS NULL OR last_used_at_ms >= created_at_ms
    ),
    backup_counter_anomaly_at_ms BIGINT CHECK (
        backup_counter_anomaly_at_ms IS NULL OR (
            backup_counter_anomaly_at_ms >= created_at_ms AND backup_eligible
        )
    ),
    revoked_at_ms BIGINT CHECK (revoked_at_ms IS NULL OR revoked_at_ms >= created_at_ms),
    clone_suspected_at_ms BIGINT CHECK (
        clone_suspected_at_ms IS NULL OR clone_suspected_at_ms >= created_at_ms
    ),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    CHECK (
        (status = 'active' AND revoked_at_ms IS NULL AND clone_suspected_at_ms IS NULL)
        OR (status = 'revoked' AND revoked_at_ms IS NOT NULL AND clone_suspected_at_ms IS NULL)
        OR (
            status = 'clone_suspected'
            AND revoked_at_ms IS NULL AND clone_suspected_at_ms IS NOT NULL
        )
    )
);

CREATE TABLE webauthn_credential_transports (
    credential_id UUID NOT NULL REFERENCES webauthn_credentials(id) ON DELETE CASCADE,
    transport TEXT NOT NULL CHECK (transport IN ('usb','nfc','ble','internal','hybrid','test')),
    PRIMARY KEY (credential_id, transport)
);

CREATE TABLE webauthn_ceremonies (
    id UUID PRIMARY KEY,
    kind TEXT NOT NULL CHECK (kind IN ('registration','authentication')),
    status TEXT NOT NULL CHECK (status IN ('pending','consumed','rejected','expired')),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_id UUID REFERENCES auth_sessions(id) ON DELETE RESTRICT,
    purpose TEXT NOT NULL CHECK (purpose IN (
        'login','reauthenticate','sensitive_action','credential_enrollment'
    )),
    rp_id TEXT NOT NULL CHECK (
        char_length(rp_id) BETWEEN 1 AND 253
        AND rp_id = lower(rp_id) AND position('/' IN rp_id) = 0 AND position(':' IN rp_id) = 0
    ),
    origin TEXT NOT NULL CHECK (
        char_length(origin) BETWEEN 9 AND 2048
        AND left(origin, 8) = 'https://' AND position('/' IN substring(origin FROM 9)) = 0
    ),
    user_revision BIGINT CHECK (user_revision IS NULL OR user_revision >= 0),
    auth_revision BIGINT NOT NULL CHECK (auth_revision >= 0),
    recent_auth_at_ms BIGINT CHECK (recent_auth_at_ms IS NULL OR recent_auth_at_ms >= 0),
    auth_challenge_id UUID REFERENCES auth_challenges(id) ON DELETE CASCADE,
    claim_id UUID UNIQUE,
    reserved_at_ms BIGINT CHECK (reserved_at_ms IS NULL OR reserved_at_ms >= 0),
    verification_expires_at_ms BIGINT CHECK (
        verification_expires_at_ms IS NULL OR verification_expires_at_ms > reserved_at_ms
    ),
    context_key_version INTEGER CHECK (context_key_version IS NULL OR context_key_version > 0),
    client_network_hmac BYTEA CHECK (
        client_network_hmac IS NULL OR octet_length(client_network_hmac) = 32
    ),
    user_agent_hash BYTEA CHECK (
        user_agent_hash IS NULL OR octet_length(user_agent_hash) = 32
    ),
    state_schema_version INTEGER CHECK (
        state_schema_version IS NULL OR state_schema_version = 1
    ),
    state_key_version INTEGER CHECK (state_key_version IS NULL OR state_key_version > 0),
    state_nonce BYTEA CHECK (state_nonce IS NULL OR octet_length(state_nonce) = 24),
    state_ciphertext BYTEA CHECK (
        state_ciphertext IS NULL OR octet_length(state_ciphertext) BETWEEN 17 AND 262144
    ),
    state_aad_hash BYTEA CHECK (state_aad_hash IS NULL OR octet_length(state_aad_hash) = 32),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    expires_at_ms BIGINT NOT NULL CHECK (expires_at_ms > created_at_ms),
    finished_at_ms BIGINT CHECK (
        finished_at_ms IS NULL OR (
            finished_at_ms >= created_at_ms AND finished_at_ms <= expires_at_ms
        )
    ),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    CHECK (
        (context_key_version IS NULL AND client_network_hmac IS NULL AND user_agent_hash IS NULL)
        OR
        (context_key_version IS NOT NULL AND client_network_hmac IS NOT NULL AND user_agent_hash IS NOT NULL)
    ),
    CHECK (
        (status = 'pending' AND finished_at_ms IS NULL
            AND state_schema_version = 1 AND state_key_version IS NOT NULL
            AND state_nonce IS NOT NULL AND state_ciphertext IS NOT NULL
            AND state_aad_hash IS NOT NULL)
        OR
        (status IN ('consumed','rejected','expired') AND finished_at_ms IS NOT NULL
            AND state_schema_version IS NULL AND state_key_version IS NULL
            AND state_nonce IS NULL AND state_ciphertext IS NULL AND state_aad_hash IS NULL)
    ),
    CHECK (
        (kind = 'registration' AND session_id IS NOT NULL
            AND purpose = 'credential_enrollment'
            AND user_revision IS NOT NULL AND recent_auth_at_ms IS NOT NULL
            AND auth_challenge_id IS NULL AND claim_id IS NULL
            AND reserved_at_ms IS NULL AND verification_expires_at_ms IS NULL
            AND context_key_version IS NULL)
        OR
        (kind = 'authentication'
            AND user_revision IS NULL AND recent_auth_at_ms IS NULL
            AND auth_challenge_id IS NOT NULL AND claim_id IS NOT NULL
            AND reserved_at_ms IS NOT NULL AND verification_expires_at_ms IS NOT NULL
            AND created_at_ms >= reserved_at_ms
            AND expires_at_ms = verification_expires_at_ms)
    )
);

CREATE UNIQUE INDEX webauthn_registration_one_pending_uq
    ON webauthn_ceremonies(user_id, session_id, purpose)
    WHERE kind = 'registration' AND status = 'pending';

CREATE INDEX webauthn_ceremonies_expiry_idx
    ON webauthn_ceremonies(status, expires_at_ms, id)
    WHERE status = 'pending';

CREATE INDEX webauthn_credentials_user_status_idx
    ON webauthn_credentials(user_id, status, created_at_ms, id);
