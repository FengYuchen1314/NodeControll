CREATE TABLE users (
    id TEXT PRIMARY KEY CHECK (
        length(id) = 36
        AND id GLOB '????????-????-????-????-????????????'
        AND id NOT GLOB '*[^0-9A-Fa-f-]*'
    ),
    username TEXT NOT NULL CHECK (
        length(username) BETWEEN 3 AND 32
        AND username NOT GLOB '*[^A-Za-z0-9_.-]*'
    ),
    username_norm TEXT NOT NULL CHECK (
        length(username_norm) BETWEEN 3 AND 32
        AND username_norm NOT GLOB '*[^a-z0-9_.-]*'
        AND username_norm = lower(username)
    ),
    password_hash TEXT NOT NULL CHECK (length(password_hash) BETWEEN 1 AND 512),
    role TEXT NOT NULL CHECK (role IN ('owner','admin','operator','support','auditor','member')),
    status TEXT NOT NULL CHECK (status IN ('active','disabled','suspended')),
    principal_label TEXT NOT NULL UNIQUE CHECK (
        length(principal_label) BETWEEN 1 AND 80
        AND principal_label NOT GLOB '*[^A-Za-z0-9_.-]*'
    ),
    force_password_change INTEGER NOT NULL CHECK (force_password_change IN (0,1)),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    deleted_at_ms INTEGER CHECK (deleted_at_ms IS NULL OR deleted_at_ms >= created_at_ms)
);

CREATE UNIQUE INDEX users_username_norm_active_uq
    ON users(username_norm)
    WHERE deleted_at_ms IS NULL;

ALTER TABLE instance_settings RENAME TO instance_settings_without_user_fk;

CREATE TABLE instance_settings (
    instance_id TEXT NOT NULL REFERENCES instances(id) ON DELETE CASCADE,
    key TEXT NOT NULL CHECK (length(key) BETWEEN 1 AND 120),
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    value_json TEXT NOT NULL CHECK (json_valid(value_json)),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    updated_by TEXT REFERENCES users(id) ON DELETE SET NULL,
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms >= 0),
    PRIMARY KEY (instance_id, key)
);

INSERT INTO instance_settings (
    instance_id, key, schema_version, value_json, revision, updated_by, updated_at_ms
)
SELECT
    instance_id, key, schema_version, value_json, revision, NULL, updated_at_ms
FROM instance_settings_without_user_fk;

DROP TABLE instance_settings_without_user_fk;

CREATE TABLE control_plane_bootstrap (
    singleton_key INTEGER PRIMARY KEY CHECK (singleton_key = 1),
    status TEXT NOT NULL CHECK (status IN ('pending','ready')),
    instance_id TEXT REFERENCES instances(id) ON DELETE RESTRICT,
    CHECK (status = 'pending' OR instance_id IS NOT NULL)
);

INSERT INTO control_plane_bootstrap (singleton_key, status, instance_id)
SELECT 1, 'pending', (SELECT id FROM instances WHERE singleton_key = 1);
