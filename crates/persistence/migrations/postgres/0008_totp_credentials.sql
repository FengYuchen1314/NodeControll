-- WP02-C4: encrypted TOTP enrollment, activation, replay prevention, and disable lifecycle.
-- Seed plaintext is never stored here: secret_record_id references a typed AEAD envelope.
-- A replacement enrollment must coexist with the still-active seed until activation commits.
-- Other secret purposes retain the C2 one-live-envelope invariant.
DROP INDEX secret_records_active_binding_uq;

CREATE UNIQUE INDEX secret_records_active_binding_uq
    ON secret_records(owner_type, owner_id, purpose)
    WHERE deleted_at_ms IS NULL AND purpose <> 'totp_seed';

CREATE TABLE totp_credentials (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    secret_record_id UUID NOT NULL UNIQUE REFERENCES secret_records(id) ON DELETE RESTRICT,
    status TEXT NOT NULL CHECK (status IN ('pending','active','disabled')),
    algorithm TEXT NOT NULL DEFAULT 'sha1' CHECK (algorithm = 'sha1'),
    digits SMALLINT NOT NULL DEFAULT 6 CHECK (digits = 6),
    period_seconds INTEGER NOT NULL DEFAULT 30 CHECK (period_seconds = 30),
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    pending_expires_at_ms BIGINT CHECK (
        pending_expires_at_ms IS NULL OR pending_expires_at_ms > created_at_ms
    ),
    activated_at_ms BIGINT CHECK (
        activated_at_ms IS NULL OR activated_at_ms >= created_at_ms
    ),
    disabled_at_ms BIGINT CHECK (
        disabled_at_ms IS NULL OR disabled_at_ms >= created_at_ms
    ),
    last_accepted_step BIGINT CHECK (
        last_accepted_step IS NULL OR last_accepted_step >= 0
    ),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
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
