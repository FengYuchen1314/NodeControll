-- WP02-C4: encrypted TOTP enrollment, activation, replay prevention, and disable lifecycle.
-- Seed plaintext is never stored here: secret_record_id references a typed AEAD envelope.
-- A replacement enrollment must coexist with the still-active seed until activation commits.
-- Other secret purposes retain the C2 one-live-envelope invariant.
DROP INDEX secret_records_active_binding_uq;

CREATE UNIQUE INDEX secret_records_active_binding_uq
    ON secret_records(owner_type, owner_id, purpose)
    WHERE deleted_at_ms IS NULL AND purpose <> 'totp_seed';

CREATE TABLE totp_credentials (
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
    secret_record_id TEXT NOT NULL UNIQUE REFERENCES secret_records(id) ON DELETE RESTRICT CHECK (
        typeof(secret_record_id) = 'text' AND length(secret_record_id) = 36
        AND substr(secret_record_id, 9, 1) = '-' AND substr(secret_record_id, 14, 1) = '-'
        AND substr(secret_record_id, 19, 1) = '-' AND substr(secret_record_id, 24, 1) = '-'
        AND length(replace(secret_record_id, '-', '')) = 32
        AND replace(secret_record_id, '-', '') NOT GLOB '*[^0-9a-f]*'
    ),
    status TEXT NOT NULL CHECK (
        typeof(status) = 'text' AND status IN ('pending','active','disabled')
    ),
    algorithm TEXT NOT NULL DEFAULT 'sha1' CHECK (
        typeof(algorithm) = 'text' AND algorithm = 'sha1'
    ),
    digits INTEGER NOT NULL DEFAULT 6 CHECK (
        typeof(digits) = 'integer' AND digits = 6
    ),
    period_seconds INTEGER NOT NULL DEFAULT 30 CHECK (
        typeof(period_seconds) = 'integer' AND period_seconds = 30
    ),
    created_at_ms INTEGER NOT NULL CHECK (
        typeof(created_at_ms) = 'integer' AND created_at_ms >= 0
    ),
    pending_expires_at_ms INTEGER CHECK (
        pending_expires_at_ms IS NULL OR (
            typeof(pending_expires_at_ms) = 'integer'
            AND pending_expires_at_ms > created_at_ms
        )
    ),
    activated_at_ms INTEGER CHECK (
        activated_at_ms IS NULL OR (
            typeof(activated_at_ms) = 'integer' AND activated_at_ms >= created_at_ms
        )
    ),
    disabled_at_ms INTEGER CHECK (
        disabled_at_ms IS NULL OR (
            typeof(disabled_at_ms) = 'integer' AND disabled_at_ms >= created_at_ms
        )
    ),
    last_accepted_step INTEGER CHECK (
        last_accepted_step IS NULL OR (
            typeof(last_accepted_step) = 'integer' AND last_accepted_step >= 0
        )
    ),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (
        typeof(revision) = 'integer' AND revision >= 0
    ),
    CHECK (
        (status = 'pending'
            AND pending_expires_at_ms IS NOT NULL
            AND activated_at_ms IS NULL AND disabled_at_ms IS NULL
            AND last_accepted_step IS NULL)
        OR
        (status = 'active'
            AND pending_expires_at_ms IS NULL
            AND activated_at_ms IS NOT NULL AND disabled_at_ms IS NULL
            AND last_accepted_step IS NOT NULL)
        OR
        (status = 'disabled'
            AND pending_expires_at_ms IS NULL AND disabled_at_ms IS NOT NULL
            AND (
                (activated_at_ms IS NULL AND last_accepted_step IS NULL)
                OR (activated_at_ms IS NOT NULL AND last_accepted_step IS NOT NULL)
            ))
    )
);

CREATE UNIQUE INDEX totp_credentials_one_pending_uq
    ON totp_credentials(user_id)
    WHERE status = 'pending';

CREATE UNIQUE INDEX totp_credentials_one_active_uq
    ON totp_credentials(user_id)
    WHERE status = 'active';

CREATE INDEX totp_credentials_user_status_idx
    ON totp_credentials(user_id, status, created_at_ms DESC, id);

CREATE INDEX totp_credentials_pending_expiry_idx
    ON totp_credentials(pending_expires_at_ms, id)
    WHERE status = 'pending';
