CREATE TABLE instances (
    singleton_key SMALLINT NOT NULL DEFAULT 1 UNIQUE CHECK (singleton_key = 1),
    id UUID PRIMARY KEY,
    name TEXT NOT NULL CHECK (char_length(btrim(name)) BETWEEN 1 AND 80),
    public_id UUID NOT NULL UNIQUE,
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0)
);

CREATE TABLE instance_settings (
    instance_id UUID NOT NULL REFERENCES instances(id) ON DELETE CASCADE,
    key TEXT NOT NULL CHECK (char_length(key) BETWEEN 1 AND 120),
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    value_json JSONB NOT NULL,
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    updated_by UUID,
    updated_at_ms BIGINT NOT NULL CHECK (updated_at_ms >= 0),
    PRIMARY KEY (instance_id, key)
);

CREATE TABLE secret_records (
    id UUID PRIMARY KEY,
    purpose TEXT NOT NULL CHECK (char_length(purpose) BETWEEN 1 AND 120),
    key_version INTEGER NOT NULL CHECK (key_version > 0),
    nonce BYTEA NOT NULL CHECK (octet_length(nonce) >= 12),
    ciphertext BYTEA NOT NULL CHECK (octet_length(ciphertext) > 0),
    aad_hash BYTEA NOT NULL CHECK (octet_length(aad_hash) = 32),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    rotated_from UUID REFERENCES secret_records(id),
    deleted_at_ms BIGINT CHECK (deleted_at_ms IS NULL OR deleted_at_ms >= created_at_ms)
);

CREATE TABLE content_objects (
    id UUID PRIMARY KEY,
    sha256 TEXT NOT NULL UNIQUE CHECK (char_length(sha256) = 64),
    size_bytes BIGINT NOT NULL CHECK (size_bytes >= 0),
    mime TEXT NOT NULL CHECK (char_length(mime) BETWEEN 1 AND 255),
    storage_backend TEXT NOT NULL CHECK (storage_backend IN ('filesystem','s3')),
    storage_key TEXT NOT NULL UNIQUE CHECK (char_length(storage_key) BETWEEN 1 AND 1024),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    ref_count BIGINT NOT NULL DEFAULT 0 CHECK (ref_count >= 0)
);

CREATE TABLE content_references (
    object_id UUID NOT NULL REFERENCES content_objects(id) ON DELETE RESTRICT,
    owner_type TEXT NOT NULL CHECK (char_length(owner_type) BETWEEN 1 AND 80),
    owner_id UUID NOT NULL,
    purpose TEXT NOT NULL CHECK (char_length(purpose) BETWEEN 1 AND 120),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    PRIMARY KEY (owner_type, owner_id, purpose)
);

CREATE INDEX content_references_object_idx ON content_references(object_id);

