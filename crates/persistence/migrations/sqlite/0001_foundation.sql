CREATE TABLE instances (
    singleton_key INTEGER NOT NULL DEFAULT 1 UNIQUE CHECK (singleton_key = 1),
    id TEXT PRIMARY KEY CHECK (length(id) = 36),
    name TEXT NOT NULL CHECK (length(trim(name)) BETWEEN 1 AND 80),
    public_id TEXT NOT NULL UNIQUE CHECK (length(public_id) = 36),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0)
);

CREATE TABLE instance_settings (
    instance_id TEXT NOT NULL REFERENCES instances(id) ON DELETE CASCADE,
    key TEXT NOT NULL CHECK (length(key) BETWEEN 1 AND 120),
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    value_json TEXT NOT NULL CHECK (json_valid(value_json)),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    updated_by TEXT,
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms >= 0),
    PRIMARY KEY (instance_id, key)
);

CREATE TABLE secret_records (
    id TEXT PRIMARY KEY CHECK (length(id) = 36),
    purpose TEXT NOT NULL CHECK (length(purpose) BETWEEN 1 AND 120),
    key_version INTEGER NOT NULL CHECK (key_version > 0),
    nonce BLOB NOT NULL CHECK (length(nonce) >= 12),
    ciphertext BLOB NOT NULL CHECK (length(ciphertext) > 0),
    aad_hash BLOB NOT NULL CHECK (length(aad_hash) = 32),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    rotated_from TEXT REFERENCES secret_records(id),
    deleted_at_ms INTEGER CHECK (deleted_at_ms IS NULL OR deleted_at_ms >= created_at_ms)
);

CREATE TABLE content_objects (
    id TEXT PRIMARY KEY CHECK (length(id) = 36),
    sha256 TEXT NOT NULL UNIQUE CHECK (length(sha256) = 64),
    size_bytes INTEGER NOT NULL CHECK (size_bytes >= 0),
    mime TEXT NOT NULL CHECK (length(mime) BETWEEN 1 AND 255),
    storage_backend TEXT NOT NULL CHECK (storage_backend IN ('filesystem','s3')),
    storage_key TEXT NOT NULL UNIQUE CHECK (length(storage_key) BETWEEN 1 AND 1024),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    ref_count INTEGER NOT NULL DEFAULT 0 CHECK (ref_count >= 0)
);

CREATE TABLE content_references (
    object_id TEXT NOT NULL REFERENCES content_objects(id) ON DELETE RESTRICT,
    owner_type TEXT NOT NULL CHECK (length(owner_type) BETWEEN 1 AND 80),
    owner_id TEXT NOT NULL CHECK (length(owner_id) = 36),
    purpose TEXT NOT NULL CHECK (length(purpose) BETWEEN 1 AND 120),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    PRIMARY KEY (owner_type, owner_id, purpose)
);

CREATE INDEX content_references_object_idx ON content_references(object_id);

