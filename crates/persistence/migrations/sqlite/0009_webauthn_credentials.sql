-- WP02-C5: encrypted, typed WebAuthn ceremony state and multi-passkey lifecycle.
-- Raw attestation objects, clientDataJSON and authenticatorData are deliberately absent.
CREATE TABLE webauthn_credentials (
    id TEXT PRIMARY KEY CHECK (
        typeof(id) = 'text' AND length(id) = 36
        AND substr(id, 9, 1) = '-' AND substr(id, 14, 1) = '-'
        AND substr(id, 19, 1) = '-' AND substr(id, 24, 1) = '-'
        AND length(replace(id, '-', '')) = 32
        AND replace(id, '-', '') NOT GLOB '*[^0-9a-f]*'
    ),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE RESTRICT CHECK (
        typeof(user_id) = 'text' AND length(user_id) = 36
        AND substr(user_id, 9, 1) = '-' AND substr(user_id, 14, 1) = '-'
        AND substr(user_id, 19, 1) = '-' AND substr(user_id, 24, 1) = '-'
        AND length(replace(user_id, '-', '')) = 32
        AND replace(user_id, '-', '') NOT GLOB '*[^0-9a-f]*'
    ),
    credential_id BLOB NOT NULL UNIQUE CHECK (
        typeof(credential_id) = 'blob' AND length(credential_id) BETWEEN 16 AND 1023
    ),
    user_handle BLOB NOT NULL CHECK (
        typeof(user_handle) = 'blob' AND length(user_handle) = 16
    ),
    aaguid BLOB CHECK (
        aaguid IS NULL OR (typeof(aaguid) = 'blob' AND length(aaguid) = 16)
    ),
    user_verified INTEGER NOT NULL CHECK (
        typeof(user_verified) = 'integer' AND user_verified = 1
    ),
    backup_eligible INTEGER NOT NULL CHECK (
        typeof(backup_eligible) = 'integer' AND backup_eligible IN (0, 1)
    ),
    backup_state INTEGER NOT NULL CHECK (
        typeof(backup_state) = 'integer' AND backup_state IN (0, 1)
        AND (backup_state = 0 OR backup_eligible = 1)
    ),
    sign_counter INTEGER NOT NULL CHECK (
        typeof(sign_counter) = 'integer' AND sign_counter BETWEEN 0 AND 4294967295
    ),
    nickname TEXT NOT NULL CHECK (
        typeof(nickname) = 'text' AND length(trim(nickname)) BETWEEN 1 AND 80
        AND nickname = trim(nickname)
    ),
    status TEXT NOT NULL CHECK (
        typeof(status) = 'text' AND status IN ('active','revoked','clone_suspected')
    ),
    material_schema_version INTEGER NOT NULL DEFAULT 1 CHECK (
        typeof(material_schema_version) = 'integer' AND material_schema_version = 1
    ),
    material_key_version INTEGER NOT NULL CHECK (
        typeof(material_key_version) = 'integer'
        AND material_key_version BETWEEN 1 AND 2147483647
    ),
    material_nonce BLOB NOT NULL CHECK (
        typeof(material_nonce) = 'blob' AND length(material_nonce) = 24
    ),
    material_ciphertext BLOB NOT NULL CHECK (
        typeof(material_ciphertext) = 'blob'
        AND length(material_ciphertext) BETWEEN 17 AND 262144
    ),
    material_aad_hash BLOB NOT NULL CHECK (
        typeof(material_aad_hash) = 'blob' AND length(material_aad_hash) = 32
    ),
    created_at_ms INTEGER NOT NULL CHECK (
        typeof(created_at_ms) = 'integer' AND created_at_ms >= 0
    ),
    last_used_at_ms INTEGER CHECK (
        last_used_at_ms IS NULL OR (
            typeof(last_used_at_ms) = 'integer' AND last_used_at_ms >= created_at_ms
        )
    ),
    backup_counter_anomaly_at_ms INTEGER CHECK (
        backup_counter_anomaly_at_ms IS NULL OR (
            typeof(backup_counter_anomaly_at_ms) = 'integer'
            AND backup_counter_anomaly_at_ms >= created_at_ms
            AND backup_eligible = 1
        )
    ),
    revoked_at_ms INTEGER CHECK (
        revoked_at_ms IS NULL OR (
            typeof(revoked_at_ms) = 'integer' AND revoked_at_ms >= created_at_ms
        )
    ),
    clone_suspected_at_ms INTEGER CHECK (
        clone_suspected_at_ms IS NULL OR (
            typeof(clone_suspected_at_ms) = 'integer'
            AND clone_suspected_at_ms >= created_at_ms
        )
    ),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (
        typeof(revision) = 'integer' AND revision >= 0
    ),
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
    credential_id TEXT NOT NULL REFERENCES webauthn_credentials(id) ON DELETE CASCADE CHECK (
        typeof(credential_id) = 'text' AND length(credential_id) = 36
        AND substr(credential_id, 9, 1) = '-' AND substr(credential_id, 14, 1) = '-'
        AND substr(credential_id, 19, 1) = '-' AND substr(credential_id, 24, 1) = '-'
        AND length(replace(credential_id, '-', '')) = 32
        AND replace(credential_id, '-', '') NOT GLOB '*[^0-9a-f]*'
    ),
    transport TEXT NOT NULL CHECK (
        typeof(transport) = 'text'
        AND transport IN ('usb','nfc','ble','internal','hybrid','test')
    ),
    PRIMARY KEY (credential_id, transport)
);

CREATE TABLE webauthn_ceremonies (
    id TEXT PRIMARY KEY CHECK (
        typeof(id) = 'text' AND length(id) = 36
        AND substr(id, 9, 1) = '-' AND substr(id, 14, 1) = '-'
        AND substr(id, 19, 1) = '-' AND substr(id, 24, 1) = '-'
        AND length(replace(id, '-', '')) = 32
        AND replace(id, '-', '') NOT GLOB '*[^0-9a-f]*'
    ),
    kind TEXT NOT NULL CHECK (
        typeof(kind) = 'text' AND kind IN ('registration','authentication')
    ),
    status TEXT NOT NULL CHECK (
        typeof(status) = 'text' AND status IN ('pending','consumed','rejected','expired')
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
    purpose TEXT NOT NULL CHECK (
        typeof(purpose) = 'text'
        AND purpose IN ('login','reauthenticate','sensitive_action','credential_enrollment')
    ),
    rp_id TEXT NOT NULL CHECK (
        typeof(rp_id) = 'text' AND length(rp_id) BETWEEN 1 AND 253
        AND rp_id = lower(rp_id) AND instr(rp_id, '/') = 0 AND instr(rp_id, ':') = 0
    ),
    origin TEXT NOT NULL CHECK (
        typeof(origin) = 'text' AND length(origin) BETWEEN 9 AND 2048
        AND substr(origin, 1, 8) = 'https://' AND instr(substr(origin, 9), '/') = 0
    ),
    user_revision INTEGER CHECK (
        user_revision IS NULL OR (typeof(user_revision) = 'integer' AND user_revision >= 0)
    ),
    auth_revision INTEGER NOT NULL CHECK (
        typeof(auth_revision) = 'integer' AND auth_revision >= 0
    ),
    recent_auth_at_ms INTEGER CHECK (
        recent_auth_at_ms IS NULL OR (
            typeof(recent_auth_at_ms) = 'integer' AND recent_auth_at_ms >= 0
        )
    ),
    auth_challenge_id TEXT REFERENCES auth_challenges(id) ON DELETE CASCADE CHECK (
        auth_challenge_id IS NULL OR (
            typeof(auth_challenge_id) = 'text' AND length(auth_challenge_id) = 36
            AND substr(auth_challenge_id, 9, 1) = '-'
            AND substr(auth_challenge_id, 14, 1) = '-'
            AND substr(auth_challenge_id, 19, 1) = '-'
            AND substr(auth_challenge_id, 24, 1) = '-'
            AND length(replace(auth_challenge_id, '-', '')) = 32
            AND replace(auth_challenge_id, '-', '') NOT GLOB '*[^0-9a-f]*'
        )
    ),
    claim_id TEXT UNIQUE CHECK (
        claim_id IS NULL OR (
            typeof(claim_id) = 'text' AND length(claim_id) = 36
            AND substr(claim_id, 9, 1) = '-' AND substr(claim_id, 14, 1) = '-'
            AND substr(claim_id, 19, 1) = '-' AND substr(claim_id, 24, 1) = '-'
            AND length(replace(claim_id, '-', '')) = 32
            AND replace(claim_id, '-', '') NOT GLOB '*[^0-9a-f]*'
        )
    ),
    reserved_at_ms INTEGER CHECK (
        reserved_at_ms IS NULL OR (
            typeof(reserved_at_ms) = 'integer' AND reserved_at_ms >= 0
        )
    ),
    verification_expires_at_ms INTEGER CHECK (
        verification_expires_at_ms IS NULL OR (
            typeof(verification_expires_at_ms) = 'integer'
            AND verification_expires_at_ms > reserved_at_ms
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
    state_schema_version INTEGER CHECK (
        state_schema_version IS NULL OR (
            typeof(state_schema_version) = 'integer' AND state_schema_version = 1
        )
    ),
    state_key_version INTEGER CHECK (
        state_key_version IS NULL OR (
            typeof(state_key_version) = 'integer'
            AND state_key_version BETWEEN 1 AND 2147483647
        )
    ),
    state_nonce BLOB CHECK (
        state_nonce IS NULL OR (typeof(state_nonce) = 'blob' AND length(state_nonce) = 24)
    ),
    state_ciphertext BLOB CHECK (
        state_ciphertext IS NULL OR (
            typeof(state_ciphertext) = 'blob'
            AND length(state_ciphertext) BETWEEN 17 AND 262144
        )
    ),
    state_aad_hash BLOB CHECK (
        state_aad_hash IS NULL OR (
            typeof(state_aad_hash) = 'blob' AND length(state_aad_hash) = 32
        )
    ),
    created_at_ms INTEGER NOT NULL CHECK (
        typeof(created_at_ms) = 'integer' AND created_at_ms >= 0
    ),
    expires_at_ms INTEGER NOT NULL CHECK (
        typeof(expires_at_ms) = 'integer' AND expires_at_ms > created_at_ms
    ),
    finished_at_ms INTEGER CHECK (
        finished_at_ms IS NULL OR (
            typeof(finished_at_ms) = 'integer' AND finished_at_ms >= created_at_ms
            AND finished_at_ms <= expires_at_ms
        )
    ),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (
        typeof(revision) = 'integer' AND revision >= 0
    ),
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
