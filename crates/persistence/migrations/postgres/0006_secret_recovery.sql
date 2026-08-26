-- WP02-C2: typed encrypted-secret records and single-use recovery-code sets.
-- Untyped v1 records cannot be assigned an authenticated owner/schema after the fact.
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM secret_records) THEN
        RAISE EXCEPTION 'cannot migrate non-empty untyped secret_records; re-encrypt records first';
    END IF;
END $$;

DROP TABLE secret_records;

CREATE TABLE secret_records (
    id UUID PRIMARY KEY,
    owner_type TEXT NOT NULL CHECK (owner_type IN ('system','instance','user')),
    owner_id UUID NOT NULL,
    purpose TEXT NOT NULL CHECK (purpose IN ('root_key_canary','totp_seed')),
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    key_version INTEGER NOT NULL CHECK (key_version > 0),
    nonce BYTEA NOT NULL CHECK (octet_length(nonce) = 24),
    ciphertext BYTEA NOT NULL CHECK (octet_length(ciphertext) > 0),
    aad_hash BYTEA NOT NULL CHECK (octet_length(aad_hash) = 32),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    rotated_from UUID REFERENCES secret_records(id) ON DELETE RESTRICT,
    deleted_at_ms BIGINT CHECK (deleted_at_ms IS NULL OR deleted_at_ms >= created_at_ms),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    CHECK (
        (owner_type = 'system' AND owner_id = '00000000-0000-0000-0000-000000000000'::uuid) OR
        (owner_type <> 'system' AND owner_id <> '00000000-0000-0000-0000-000000000000'::uuid)
    ),
    CHECK (purpose <> 'root_key_canary' OR owner_type = 'system'),
    CHECK (purpose <> 'totp_seed' OR owner_type = 'user')
);

CREATE UNIQUE INDEX secret_records_active_binding_uq
ON secret_records(owner_type, owner_id, purpose)
WHERE deleted_at_ms IS NULL;

CREATE INDEX secret_records_key_version_idx
ON secret_records(key_version)
WHERE deleted_at_ms IS NULL;

CREATE TABLE recovery_code_sets (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    set_version BIGINT NOT NULL CHECK (set_version > 0),
    status TEXT NOT NULL CHECK (status IN ('active','replaced')),
    total_count SMALLINT NOT NULL CHECK (total_count = 8),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    replaced_at_ms BIGINT CHECK (
        (status = 'active' AND replaced_at_ms IS NULL) OR
        (status = 'replaced' AND replaced_at_ms >= created_at_ms)
    ),
    PRIMARY KEY (user_id, set_version)
);

CREATE UNIQUE INDEX recovery_code_sets_one_active_uq
ON recovery_code_sets(user_id)
WHERE status = 'active';

CREATE TABLE recovery_codes (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    set_version BIGINT NOT NULL CHECK (set_version > 0),
    position SMALLINT NOT NULL CHECK (position BETWEEN 1 AND 8),
    digest_key_version INTEGER NOT NULL CHECK (digest_key_version > 0),
    code_hmac BYTEA NOT NULL CHECK (octet_length(code_hmac) = 32),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    consumed_at_ms BIGINT CHECK (consumed_at_ms IS NULL OR consumed_at_ms >= created_at_ms),
    FOREIGN KEY (user_id, set_version) REFERENCES recovery_code_sets(user_id, set_version) ON DELETE RESTRICT,
    UNIQUE (user_id, set_version, position),
    UNIQUE (digest_key_version, code_hmac)
);

CREATE INDEX recovery_codes_remaining_idx
ON recovery_codes(user_id, set_version, digest_key_version, code_hmac)
WHERE consumed_at_ms IS NULL;
