CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL CHECK (username ~ '^[A-Za-z0-9_.-]{3,32}$'),
    username_norm TEXT NOT NULL CHECK (
        username_norm ~ '^[a-z0-9_.-]{3,32}$'
        AND username_norm = lower(username)
    ),
    password_hash TEXT NOT NULL CHECK (char_length(password_hash) BETWEEN 1 AND 512),
    role TEXT NOT NULL CHECK (role IN ('owner','admin','operator','support','auditor','member')),
    status TEXT NOT NULL CHECK (status IN ('active','disabled','suspended')),
    principal_label TEXT NOT NULL UNIQUE CHECK (principal_label ~ '^[A-Za-z0-9_.-]{1,80}$'),
    force_password_change BOOLEAN NOT NULL,
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    deleted_at_ms BIGINT CHECK (deleted_at_ms IS NULL OR deleted_at_ms >= created_at_ms)
);

CREATE UNIQUE INDEX users_username_norm_active_uq
    ON users(username_norm)
    WHERE deleted_at_ms IS NULL;

UPDATE instance_settings SET updated_by = NULL WHERE updated_by IS NOT NULL;

ALTER TABLE instance_settings
    ADD CONSTRAINT instance_settings_updated_by_fk
    FOREIGN KEY (updated_by) REFERENCES users(id) ON DELETE SET NULL;

CREATE TABLE control_plane_bootstrap (
    singleton_key SMALLINT PRIMARY KEY CHECK (singleton_key = 1),
    status TEXT NOT NULL CHECK (status IN ('pending','ready')),
    instance_id UUID REFERENCES instances(id) ON DELETE RESTRICT,
    CHECK (status = 'pending' OR instance_id IS NOT NULL)
);

INSERT INTO control_plane_bootstrap (singleton_key, status, instance_id)
SELECT 1, 'pending', (SELECT id FROM instances WHERE singleton_key = 1);
