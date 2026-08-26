-- WP02-C2: typed encrypted-secret records and single-use recovery-code sets.
-- Untyped v1 records cannot be assigned an authenticated owner/schema after the fact. Abort the
-- migration instead of silently weakening their binding; an operator must remove/re-encrypt them.
CREATE TABLE secret_records_v1_upgrade_guard (
    legacy_row_count INTEGER NOT NULL CHECK (legacy_row_count = 0)
);
INSERT INTO secret_records_v1_upgrade_guard(legacy_row_count)
SELECT COUNT(*) FROM secret_records;
DROP TABLE secret_records_v1_upgrade_guard;

ALTER TABLE secret_records RENAME TO secret_records_v1_untyped;

CREATE TABLE secret_records (
    id TEXT PRIMARY KEY CHECK (length(id) = 36),
    owner_type TEXT NOT NULL CHECK (owner_type IN ('system','instance','user')),
    owner_id TEXT NOT NULL CHECK (length(owner_id) = 36),
    purpose TEXT NOT NULL CHECK (purpose IN ('root_key_canary','totp_seed')),
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    key_version INTEGER NOT NULL CHECK (key_version > 0 AND key_version <= 2147483647),
    nonce BLOB NOT NULL CHECK (length(nonce) = 24),
    ciphertext BLOB NOT NULL CHECK (length(ciphertext) > 0),
    aad_hash BLOB NOT NULL CHECK (length(aad_hash) = 32),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    rotated_from TEXT REFERENCES secret_records(id) ON DELETE RESTRICT,
    deleted_at_ms INTEGER CHECK (deleted_at_ms IS NULL OR deleted_at_ms >= created_at_ms),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    CHECK (
        (owner_type = 'system' AND owner_id = '00000000-0000-0000-0000-000000000000') OR
        (owner_type <> 'system' AND owner_id <> '00000000-0000-0000-0000-000000000000')
    ),
    CHECK (purpose <> 'root_key_canary' OR owner_type = 'system'),
    CHECK (purpose <> 'totp_seed' OR owner_type = 'user')
);

DROP TABLE secret_records_v1_untyped;

CREATE UNIQUE INDEX secret_records_active_binding_uq
ON secret_records(owner_type, owner_id, purpose)
WHERE deleted_at_ms IS NULL;

CREATE INDEX secret_records_key_version_idx
ON secret_records(key_version)
WHERE deleted_at_ms IS NULL;

CREATE TABLE recovery_code_sets (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    set_version INTEGER NOT NULL CHECK (set_version > 0),
    status TEXT NOT NULL CHECK (status IN ('active','replaced')),
    total_count INTEGER NOT NULL CHECK (total_count = 8),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    replaced_at_ms INTEGER CHECK (
        (status = 'active' AND replaced_at_ms IS NULL) OR
        (status = 'replaced' AND replaced_at_ms >= created_at_ms)
    ),
    PRIMARY KEY (user_id, set_version)
);

CREATE UNIQUE INDEX recovery_code_sets_one_active_uq
ON recovery_code_sets(user_id)
WHERE status = 'active';

CREATE TABLE recovery_codes (
    id TEXT PRIMARY KEY CHECK (length(id) = 36),
    user_id TEXT NOT NULL,
    set_version INTEGER NOT NULL CHECK (set_version > 0),
    position INTEGER NOT NULL CHECK (position BETWEEN 1 AND 8),
    digest_key_version INTEGER NOT NULL CHECK (digest_key_version > 0 AND digest_key_version <= 2147483647),
    code_hmac BLOB NOT NULL CHECK (length(code_hmac) = 32),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    consumed_at_ms INTEGER CHECK (consumed_at_ms IS NULL OR consumed_at_ms >= created_at_ms),
    FOREIGN KEY (user_id, set_version) REFERENCES recovery_code_sets(user_id, set_version) ON DELETE RESTRICT,
    UNIQUE (user_id, set_version, position),
    UNIQUE (digest_key_version, code_hmac)
);

CREATE INDEX recovery_codes_remaining_idx
ON recovery_codes(user_id, set_version, digest_key_version, code_hmac)
WHERE consumed_at_ms IS NULL;
