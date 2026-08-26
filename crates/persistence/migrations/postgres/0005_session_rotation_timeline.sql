ALTER TABLE auth_sessions
    DROP CONSTRAINT auth_sessions_check;

ALTER TABLE auth_sessions
    ADD CONSTRAINT auth_sessions_authenticated_at_ms_check
    CHECK (authenticated_at_ms >= 0);

ALTER TABLE auth_sessions
    DROP CONSTRAINT auth_sessions_auth_level_check;

UPDATE auth_sessions
SET auth_level = 'phishing_resistant'
WHERE auth_level = 'webauthn';

ALTER TABLE auth_sessions
    ADD CONSTRAINT auth_sessions_auth_level_check
    CHECK (auth_level IN ('password','mfa','phishing_resistant','recovery'));

ALTER TABLE auth_sessions
    DROP CONSTRAINT auth_sessions_revoked_reason_check;

ALTER TABLE auth_sessions
    ADD CONSTRAINT auth_sessions_revoked_reason_check
    CHECK (
        revoked_reason IS NULL OR revoked_reason IN (
            'logout','logout_all','password_changed','user_disabled','user_revoked',
            'administrator','rotation','expired','security_policy'
        )
    );
