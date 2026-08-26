ALTER TABLE login_security_events
    DROP CONSTRAINT login_security_events_reason_check;

ALTER TABLE login_security_events
    ADD CONSTRAINT login_security_events_reason_check CHECK (reason IN (
        'login_succeeded','invalid_credentials','rate_limited','account_inactive',
        'session_expired','session_revoked','csrf_mismatch','logout','logout_all',
        'auth_revision_changed','reauthentication_succeeded','password_changed'
    ));
