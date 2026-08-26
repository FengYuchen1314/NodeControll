use std::{str::FromStr, time::Duration};

use nodecontroll_domain::{
    AuthChallengePurpose, AuthChallengeStatus, AuthenticationAssurance, AuthenticationMethod,
    EntityId, Revision, WebAuthnCredential, WebAuthnCredentialId, WebAuthnCredentialStatus,
    WebAuthnNickname, WebAuthnOrigin, WebAuthnTransport, WebAuthnUserHandle,
};
use nodecontroll_secrets::SecretEnvelope;
use sqlx::{PgPool, Row, postgres::PgConnectOptions, postgres::PgPoolOptions};

use crate::{
    AuthChallengeAccess, AuthChallengeAttemptFailure, AuthChallengeAttemptOutcome,
    AuthChallengeAttemptReservation, AuthChallengeAttemptReservationOutcome,
    AuthChallengeAttemptResume, AuthChallengeClientContext, AuthChallengeConsumption,
    AuthChallengeConsumptionOutcome,
    BeginWebAuthnAuthenticationOutcome, BeginWebAuthnRegistrationOutcome,
    CompleteWebAuthnRegistration, CompleteWebAuthnRegistrationOutcome, ConnectionSettings,
    CreateAuthChallengeOutcome, Database, NewAuthChallenge, NewWebAuthnAuthenticationCeremony,
    NewWebAuthnCredential, NewWebAuthnRegistrationCeremony, RenameWebAuthnCredential,
    PersistenceError, RevokeWebAuthnCredential, RevokeWebAuthnCredentialOutcome,
    WebAuthnAuthenticationCommit, WebAuthnAuthenticationCommitOutcome,
    WebAuthnAuthenticationHandoff, WebAuthnChallengeBinding, WebAuthnCloneSuspected,
    WebAuthnCloneSuspectedOutcome, WebAuthnSessionGuard,
};

fn settings() -> ConnectionSettings {
    ConnectionSettings {
        max_connections: 4,
        acquire_timeout: Duration::from_secs(5),
        statement_timeout: Duration::from_secs(30),
        lock_timeout: Duration::from_secs(5),
    }
}

fn envelope(marker: u8) -> SecretEnvelope {
    SecretEnvelope {
        key_version: 1,
        nonce: [marker; 24],
        ciphertext: vec![marker.wrapping_add(1); 32],
        aad_hash: [marker.wrapping_add(2); 32],
    }
}

fn origin() -> WebAuthnOrigin {
    let parsed = WebAuthnOrigin::parse("https://control.example.test");
    assert!(parsed.is_ok());
    match parsed {
        Ok(value) => value,
        Err(_) => panic!("fixture origin must be valid"),
    }
}

fn guard(
    user_id: EntityId,
    session_id: EntityId,
    auth_revision: u64,
    recent_auth_at_ms: i64,
    now_ms: i64,
) -> WebAuthnSessionGuard {
    WebAuthnSessionGuard {
        user_id,
        actor_session_id: session_id,
        expected_user_revision: Revision::initial(),
        expected_auth_revision: Revision::from_value(auth_revision),
        expected_recent_auth_at_ms: recent_auth_at_ms,
        now_ms,
    }
}

fn credential(
    user_id: EntityId,
    marker: u8,
    created_at_ms: i64,
    counter: u32,
    backup_eligible: bool,
) -> NewWebAuthnCredential {
    let credential_id = WebAuthnCredentialId::parse(vec![marker; 32]);
    let nickname = WebAuthnNickname::parse(format!("key-{marker}"));
    assert!(credential_id.is_ok() && nickname.is_ok());
    let (Ok(credential_id), Ok(nickname)) = (credential_id, nickname) else {
        panic!("credential fixture must be valid");
    };
    NewWebAuthnCredential {
        credential: WebAuthnCredential {
            id: EntityId::new(),
            user_id,
            credential_id,
            user_handle: WebAuthnUserHandle::for_user(user_id),
            aaguid: None,
            transports: vec![WebAuthnTransport::Internal],
            user_verified: true,
            backup_eligible,
            backup_state: false,
            sign_counter: counter,
            nickname,
            status: WebAuthnCredentialStatus::Active,
            created_at_ms,
            last_used_at_ms: None,
            backup_counter_anomaly_at_ms: None,
            revoked_at_ms: None,
            clone_suspected_at_ms: None,
            revision: Revision::initial(),
        },
        material: envelope(marker),
    }
}

async fn insert_principal(
    database: &Database,
    user_id: EntityId,
    actor_session_id: EntityId,
    other_session_id: EntityId,
    marker: u8,
    recent_auth_at_ms: i64,
) {
    match database {
        Database::Sqlite(pool) => {
            let username = format!("wu-{}", &user_id.to_string()[..8]);
            assert!(
                sqlx::query(
                    "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES (?,?,?,'$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA','owner','active','webauthn-fixture',0,0,100,NULL)",
                )
                .bind(user_id.to_string())
                .bind(&username)
                .bind(&username)
                .execute(pool)
                .await
                .is_ok()
            );
            assert!(
                sqlx::query(
                    "INSERT INTO user_auth_state (user_id,auth_revision,password_changed_at_ms,updated_at_ms) VALUES (?,0,100,100)",
                )
                .bind(user_id.to_string())
                .execute(pool)
                .await
                .is_ok()
            );
            for (index, session_id) in [actor_session_id, other_session_id].into_iter().enumerate() {
                let token = [marker.wrapping_add(u8::try_from(index).unwrap_or(0)); 32];
                let csrf = [marker.wrapping_add(20).wrapping_add(u8::try_from(index).unwrap_or(0)); 32];
                assert!(
                    sqlx::query(
                        "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES (?,?,1,?,1,?,0,'password','active',100,100,?,?,2000000,3000000,NULL,NULL,NULL,NULL,NULL,0)",
                    )
                    .bind(session_id.to_string())
                    .bind(user_id.to_string())
                    .bind(token.as_slice())
                    .bind(csrf.as_slice())
                    .bind(recent_auth_at_ms)
                    .bind(recent_auth_at_ms)
                    .execute(pool)
                    .await
                    .is_ok()
                );
            }
        }
        Database::Postgres(pool) => {
            let username = format!("wu-{}", &user_id.to_string()[..8]);
            assert!(
                sqlx::query(
                    "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES ($1,$2,$2,'$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA','owner','active','webauthn-fixture',false,0,100,NULL)",
                )
                .bind(user_id.into_uuid())
                .bind(&username)
                .execute(pool)
                .await
                .is_ok()
            );
            assert!(
                sqlx::query(
                    "INSERT INTO user_auth_state (user_id,auth_revision,password_changed_at_ms,updated_at_ms) VALUES ($1,0,100,100)",
                )
                .bind(user_id.into_uuid())
                .execute(pool)
                .await
                .is_ok()
            );
            for (index, session_id) in [actor_session_id, other_session_id].into_iter().enumerate() {
                let token = [marker.wrapping_add(u8::try_from(index).unwrap_or(0)); 32];
                let csrf = [marker.wrapping_add(20).wrapping_add(u8::try_from(index).unwrap_or(0)); 32];
                assert!(
                    sqlx::query(
                        "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES ($1,$2,1,$3,1,$4,0,'password','active',100,100,$5,$5,2000000,3000000,NULL,NULL,NULL,NULL,NULL,0)",
                    )
                    .bind(session_id.into_uuid())
                    .bind(user_id.into_uuid())
                    .bind(token.as_slice())
                    .bind(csrf.as_slice())
                    .bind(recent_auth_at_ms)
                    .execute(pool)
                    .await
                    .is_ok()
                );
            }
        }
    }
}

async fn auth_revision(database: &Database, user_id: EntityId) -> i64 {
    match database {
        Database::Sqlite(pool) => {
            sqlx::query_scalar("SELECT auth_revision FROM user_auth_state WHERE user_id=?")
                .bind(user_id.to_string())
                .fetch_one(pool)
                .await
                .unwrap_or(-1)
        }
        Database::Postgres(pool) => {
            sqlx::query_scalar("SELECT auth_revision FROM user_auth_state WHERE user_id=$1")
                .bind(user_id.into_uuid())
                .fetch_one(pool)
                .await
                .unwrap_or(-1)
        }
    }
}

async fn session_status(database: &Database, session_id: EntityId) -> String {
    match database {
        Database::Sqlite(pool) => sqlx::query_scalar("SELECT status FROM auth_sessions WHERE id=?")
            .bind(session_id.to_string())
            .fetch_one(pool)
            .await
            .unwrap_or_default(),
        Database::Postgres(pool) => {
            sqlx::query_scalar("SELECT status FROM auth_sessions WHERE id=$1")
                .bind(session_id.into_uuid())
                .fetch_one(pool)
                .await
                .unwrap_or_default()
        }
    }
}

async fn credential_counter(database: &Database, credential_id: EntityId) -> i64 {
    match database {
        Database::Sqlite(pool) => {
            sqlx::query_scalar("SELECT sign_counter FROM webauthn_credentials WHERE id=?")
                .bind(credential_id.to_string())
                .fetch_one(pool)
                .await
                .unwrap_or(-1)
        }
        Database::Postgres(pool) => {
            sqlx::query_scalar("SELECT sign_counter FROM webauthn_credentials WHERE id=$1")
                .bind(credential_id.into_uuid())
                .fetch_one(pool)
                .await
                .unwrap_or(-1)
        }
    }
}

async fn ceremony_status_and_times(
    database: &Database,
    ceremony_id: EntityId,
) -> (String, i64, Option<i64>) {
    match database {
        Database::Sqlite(pool) => sqlx::query_as(
            "SELECT status,expires_at_ms,finished_at_ms FROM webauthn_ceremonies WHERE id=?",
        )
        .bind(ceremony_id.to_string())
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| panic!("ceremony fixture must remain readable")),
        Database::Postgres(pool) => sqlx::query_as(
            "SELECT status,expires_at_ms,finished_at_ms FROM webauthn_ceremonies WHERE id=$1",
        )
        .bind(ceremony_id.into_uuid())
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| panic!("ceremony fixture must remain readable")),
    }
}

async fn insert_raw_credential_id_length(
    database: &Database,
    user_id: EntityId,
    length: usize,
    marker: u8,
) -> bool {
    let credential_id = vec![marker; length];
    let nickname = format!("raw-length-{length}");
    match database {
        Database::Sqlite(pool) => sqlx::query(
            "INSERT INTO webauthn_credentials (id,user_id,credential_id,user_handle,aaguid,user_verified,backup_eligible,backup_state,sign_counter,nickname,status,material_schema_version,material_key_version,material_nonce,material_ciphertext,material_aad_hash,created_at_ms,last_used_at_ms,backup_counter_anomaly_at_ms,revoked_at_ms,clone_suspected_at_ms,revision) VALUES (?,?,?,?,NULL,1,0,0,0,?,'active',1,1,?,?,?,5000,NULL,NULL,NULL,NULL,0)",
        )
        .bind(EntityId::new().to_string())
        .bind(user_id.to_string())
        .bind(credential_id)
        .bind(user_id.into_uuid().as_bytes().as_slice())
        .bind(nickname)
        .bind([marker.wrapping_add(1); 24].as_slice())
        .bind([marker.wrapping_add(2); 32].as_slice())
        .bind([marker.wrapping_add(3); 32].as_slice())
        .execute(pool)
        .await
        .is_ok(),
        Database::Postgres(pool) => sqlx::query(
            "INSERT INTO webauthn_credentials (id,user_id,credential_id,user_handle,aaguid,user_verified,backup_eligible,backup_state,sign_counter,nickname,status,material_schema_version,material_key_version,material_nonce,material_ciphertext,material_aad_hash,created_at_ms,last_used_at_ms,backup_counter_anomaly_at_ms,revoked_at_ms,clone_suspected_at_ms,revision) VALUES ($1,$2,$3,$4,NULL,true,false,false,0,$5,'active',1,1,$6,$7,$8,5000,NULL,NULL,NULL,NULL,0)",
        )
        .bind(EntityId::new().into_uuid())
        .bind(user_id.into_uuid())
        .bind(credential_id)
        .bind(user_id.into_uuid().as_bytes().as_slice())
        .bind(nickname)
        .bind([marker.wrapping_add(1); 24].as_slice())
        .bind([marker.wrapping_add(2); 32].as_slice())
        .bind([marker.wrapping_add(3); 32].as_slice())
        .execute(pool)
        .await
        .is_ok(),
    }
}

async fn register_one(
    database: &Database,
    user_id: EntityId,
    session_id: EntityId,
    marker: u8,
    begin_at_ms: i64,
    counter: u32,
    backup_eligible: bool,
) -> NewWebAuthnCredential {
    let complete_at_ms = begin_at_ms + 100;
    let public_origin = origin();
    let enrollment = NewWebAuthnRegistrationCeremony {
        id: EntityId::new(),
        guard: guard(user_id, session_id, 0, 1_000, begin_at_ms),
        origin: public_origin.clone(),
        expires_at_ms: begin_at_ms + 100_000,
        state: envelope(marker.wrapping_add(40)),
    };
    let begun = database.begin_webauthn_registration(&enrollment).await;
    let stored = match begun {
        Ok(BeginWebAuthnRegistrationOutcome::Created(stored)) => stored,
        _ => panic!("registration fixture must begin"),
    };
    let value = credential(
        user_id,
        marker,
        complete_at_ms,
        counter,
        backup_eligible,
    );
    let complete_guard = guard(user_id, session_id, 0, 1_000, complete_at_ms);
    let completed = database
        .complete_webauthn_registration(&CompleteWebAuthnRegistration {
            ceremony_id: stored.id,
            expected_ceremony_revision: stored.revision,
            guard: complete_guard,
            origin: &public_origin,
            credential: &value,
        })
        .await;
    assert!(matches!(
        completed,
        Ok(CompleteWebAuthnRegistrationOutcome::Registered(_))
    ));
    value
}

async fn begin_bound_authentication(
    database: &Database,
    user_id: EntityId,
    session_id: EntityId,
    auth_revision: u64,
    marker: u8,
    reserved_at_ms: i64,
) -> (WebAuthnChallengeBinding, EntityId) {
    let context = AuthChallengeClientContext {
        key_version: Some(1),
        client_network_hmac: Some([marker.wrapping_add(60); 32]),
        user_agent_hash: Some([marker.wrapping_add(90); 32]),
    };
    let challenge = NewAuthChallenge {
        id: EntityId::new(),
        token_key_version: 1,
        token_hmac: [marker.wrapping_add(120); 32],
        purpose: AuthChallengePurpose::Login,
        user_id,
        session_id: Some(session_id),
        auth_revision: Revision::from_value(auth_revision),
        allowed_methods: vec![AuthenticationMethod::WebAuthn],
        max_attempts: 5,
        created_at_ms: reserved_at_ms - 10,
        expires_at_ms: reserved_at_ms + 100_000,
        rotation_required: false,
        client_context: context.clone(),
        revision: Revision::initial(),
    };
    assert!(matches!(
        database.create_auth_challenge(&challenge).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    let claim_id = EntityId::new();
    let expires_at_ms = reserved_at_ms + 90_000;
    let reserved = database
        .reserve_auth_challenge_attempt(&AuthChallengeAttemptReservation {
            access: AuthChallengeAccess {
                id: challenge.id,
                token_key_version: challenge.token_key_version,
                token_hmac: challenge.token_hmac,
                client_context: context.clone(),
                now_ms: reserved_at_ms,
            },
            claim_id,
            method: AuthenticationMethod::WebAuthn,
            expected_revision: Revision::initial(),
            verification_expires_at_ms: expires_at_ms,
        })
        .await;
    assert!(matches!(
        reserved,
        Ok(AuthChallengeAttemptReservationOutcome::Reserved(_))
    ));
    let binding = WebAuthnChallengeBinding {
        auth_challenge_id: challenge.id,
        claim_id,
        purpose: challenge.purpose,
        user_id,
        session_id: Some(session_id),
        auth_revision: challenge.auth_revision,
        reserved_at_ms,
        verification_expires_at_ms: expires_at_ms,
        client_context: context,
    };
    let ceremony = NewWebAuthnAuthenticationCeremony {
        id: EntityId::new(),
        binding: binding.clone(),
        origin: origin(),
        state: envelope(marker.wrapping_add(150)),
        created_at_ms: reserved_at_ms + 1,
    };
    let result = database.begin_webauthn_authentication(&ceremony).await;
    assert!(matches!(
        result,
        Ok(BeginWebAuthnAuthenticationOutcome::Created(_))
    ));
    (binding, ceremony.id)
}

fn authentication_access(
    binding: &WebAuthnChallengeBinding,
    marker: u8,
    now_ms: i64,
) -> AuthChallengeAccess {
    AuthChallengeAccess {
        id: binding.auth_challenge_id,
        token_key_version: 1,
        token_hmac: [marker.wrapping_add(120); 32],
        client_context: binding.client_context.clone(),
        now_ms,
    }
}

fn new_bound_challenge(
    user_id: EntityId,
    session_id: EntityId,
    auth_revision: u64,
    purpose: AuthChallengePurpose,
    marker: u8,
    created_at_ms: i64,
    expires_at_ms: i64,
) -> NewAuthChallenge {
    NewAuthChallenge {
        id: EntityId::new(),
        token_key_version: 1,
        token_hmac: [marker; 32],
        purpose,
        user_id,
        session_id: Some(session_id),
        auth_revision: Revision::from_value(auth_revision),
        allowed_methods: vec![AuthenticationMethod::WebAuthn],
        max_attempts: 5,
        created_at_ms,
        expires_at_ms,
        rotation_required: false,
        client_context: AuthChallengeClientContext::unbound(),
        revision: Revision::initial(),
    }
}

async fn schema_contract(database: &Database) {
    let forbidden = [
        "attestation_object",
        "client_data_json",
        "authenticator_data",
        "attestationObject",
        "clientDataJSON",
        "authenticatorData",
    ];
    let columns = match database {
        Database::Sqlite(pool) => {
            let mut names = Vec::new();
            for table in ["webauthn_credentials", "webauthn_ceremonies"] {
                let query = format!("PRAGMA table_info({table})");
                let rows = sqlx::query(&query).fetch_all(pool).await.unwrap_or_default();
                names.extend(
                    rows.into_iter()
                        .filter_map(|row| row.try_get::<String, _>("name").ok()),
                );
            }
            let schema: String = sqlx::query_scalar(
                "SELECT sql FROM sqlite_master WHERE type='table' AND name='webauthn_credentials'",
            )
            .fetch_one(pool)
            .await
            .unwrap_or_default();
            assert!(schema.contains("user_verified = 1"));
            assert!(schema.contains("backup_state = 0 OR backup_eligible = 1"));
            names
        }
        Database::Postgres(pool) => {
            let names: Vec<String> = sqlx::query_scalar(
                "SELECT column_name FROM information_schema.columns WHERE table_schema=current_schema() AND table_name IN ('webauthn_credentials','webauthn_ceremonies')",
            )
            .fetch_all(pool)
            .await
            .unwrap_or_default();
            names
        }
    };
    assert!(!columns.iter().any(|column| forbidden.contains(&column.as_str())));
}

async fn repository_contract(database: Database) {
    assert!(database.migrate().await.is_ok());
    schema_contract(&database).await;

    let user_id = EntityId::new();
    let actor_session_id = EntityId::new();
    let other_session_id = EntityId::new();
    insert_principal(
        &database,
        user_id,
        actor_session_id,
        other_session_id,
        10,
        1_000,
    )
    .await;
    let public_origin = origin();
    let first = NewWebAuthnRegistrationCeremony {
        id: EntityId::new(),
        guard: guard(user_id, actor_session_id, 0, 1_000, 2_000),
        origin: public_origin.clone(),
        expires_at_ms: 102_000,
        state: envelope(20),
    };
    let mut second = first.clone();
    second.id = EntityId::new();
    second.state = envelope(21);
    let (first_result, second_result) = tokio::join!(
        database.begin_webauthn_registration(&first),
        database.begin_webauthn_registration(&second),
    );
    let stored = match (first_result, second_result) {
        (
            Ok(BeginWebAuthnRegistrationOutcome::Created(stored)),
            Ok(BeginWebAuthnRegistrationOutcome::AlreadyPending),
        )
        | (
            Ok(BeginWebAuthnRegistrationOutcome::AlreadyPending),
            Ok(BeginWebAuthnRegistrationOutcome::Created(stored)),
        ) => stored,
        _ => panic!("concurrent registration must have exactly one winner"),
    };
    let wrong_origin = WebAuthnOrigin::parse("https://wrong.example.test");
    assert!(wrong_origin.is_ok());
    if let Ok(wrong_origin) = wrong_origin {
        assert!(
            database
                .webauthn_registration_ceremony(
                    stored.id,
                    stored.revision,
                    &guard(user_id, actor_session_id, 0, 1_000, 2_001),
                    &wrong_origin,
                )
                .await
                .is_ok_and(|value| value.is_none())
        );
    }
    let first_credential = credential(user_id, 30, 2_100, 0, false);
    let completed = database
        .complete_webauthn_registration(&CompleteWebAuthnRegistration {
            ceremony_id: stored.id,
            expected_ceremony_revision: stored.revision,
            guard: guard(user_id, actor_session_id, 0, 1_000, 2_100),
            origin: &public_origin,
            credential: &first_credential,
        })
        .await;
    assert!(matches!(
        completed,
        Ok(CompleteWebAuthnRegistrationOutcome::Registered(ref result))
            if result.auth_revision == Revision::from_value(1)
                && result.revoked_other_sessions == 1
    ));
    assert_eq!(session_status(&database, other_session_id).await, "revoked");
    assert_eq!(auth_revision(&database, user_id).await, 1);
    assert!(matches!(
        database
            .complete_webauthn_registration(&CompleteWebAuthnRegistration {
                ceremony_id: stored.id,
                expected_ceremony_revision: stored.revision,
                guard: guard(user_id, actor_session_id, 0, 1_000, 2_101),
                origin: &public_origin,
                credential: &first_credential,
            })
            .await,
        Ok(CompleteWebAuthnRegistrationOutcome::Stale)
    ));

    let duplicate_ceremony = NewWebAuthnRegistrationCeremony {
        id: EntityId::new(),
        guard: guard(user_id, actor_session_id, 1, 2_100, 2_200),
        origin: public_origin.clone(),
        expires_at_ms: 2_350,
        state: envelope(31),
    };
    let duplicate_started = database
        .begin_webauthn_registration(&duplicate_ceremony)
        .await;
    let duplicate_stored = match duplicate_started {
        Ok(BeginWebAuthnRegistrationOutcome::Created(stored)) => stored,
        _ => panic!("duplicate transaction fixture must begin"),
    };
    let mut duplicate = credential(user_id, 30, 2_300, 0, false);
    duplicate.credential.id = EntityId::new();
    let duplicate_result = database
        .complete_webauthn_registration(&CompleteWebAuthnRegistration {
            ceremony_id: duplicate_stored.id,
            expected_ceremony_revision: duplicate_stored.revision,
            guard: guard(user_id, actor_session_id, 1, 2_100, 2_300),
            origin: &public_origin,
            credential: &duplicate,
        })
        .await;
    assert!(matches!(
        duplicate_result,
        Ok(CompleteWebAuthnRegistrationOutcome::DuplicateCredential)
    ));
    assert_eq!(auth_revision(&database, user_id).await, 1);
    assert_eq!(
        credential_counter(&database, duplicate.credential.id).await,
        -1
    );
    assert_eq!(
        ceremony_status_and_times(&database, duplicate_stored.id).await,
        ("rejected".to_owned(), 2_350, Some(2_300))
    );
    assert!(
        database
            .webauthn_registration_ceremony(
                duplicate_stored.id,
                duplicate_stored.revision,
                &guard(user_id, actor_session_id, 1, 2_100, 2_301),
                &public_origin,
            )
            .await
            .is_ok_and(|value| value.is_none())
    );
    assert!(matches!(
        database
            .complete_webauthn_registration(&CompleteWebAuthnRegistration {
                ceremony_id: duplicate_stored.id,
                expected_ceremony_revision: duplicate_stored.revision,
                guard: guard(user_id, actor_session_id, 1, 2_100, 2_301),
                origin: &public_origin,
                credential: &duplicate,
            })
            .await,
        Ok(CompleteWebAuthnRegistrationOutcome::Stale)
    ));
    let after_duplicate = NewWebAuthnRegistrationCeremony {
        id: EntityId::new(),
        guard: guard(user_id, actor_session_id, 1, 2_100, 2_301),
        origin: public_origin.clone(),
        expires_at_ms: 2_451,
        state: envelope(33),
    };
    assert!(matches!(
        database
            .begin_webauthn_registration(&after_duplicate)
            .await,
        Ok(BeginWebAuthnRegistrationOutcome::Created(_))
    ));

    let renamed_nickname = WebAuthnNickname::parse("renamed-key");
    assert!(renamed_nickname.is_ok());
    let Ok(renamed_nickname) = renamed_nickname else {
        panic!("fixture nickname must be valid");
    };
    let renamed = database
        .rename_webauthn_credential(&RenameWebAuthnCredential {
            credential_id: first_credential.credential.id,
            expected_credential_revision: Revision::initial(),
            nickname: &renamed_nickname,
            guard: guard(user_id, actor_session_id, 1, 2_100, 2_400),
        })
        .await;
    assert!(renamed.is_ok_and(|value| {
        value.is_some_and(|value| {
            value.nickname.as_str() == "renamed-key"
                && value.revision == Revision::from_value(1)
        })
    }));

    let (binding, auth_ceremony_id) = begin_bound_authentication(
        &database,
        user_id,
        actor_session_id,
        1,
        32,
        2_500,
    )
    .await;
    let credential_id = &first_credential.credential.credential_id;
    let mut wrong_binding = binding.clone();
    wrong_binding.claim_id = EntityId::new();
    assert!(
        database
            .webauthn_authentication_context(
                auth_ceremony_id,
                Revision::initial(),
                &wrong_binding,
                &public_origin,
                credential_id,
                2_502,
            )
            .await
            .is_ok_and(|value| value.is_none())
    );
    let mut wrong_context = binding.clone();
    wrong_context.client_context.user_agent_hash = Some([255; 32]);
    assert!(
        database
            .webauthn_authentication_context(
                auth_ceremony_id,
                Revision::initial(),
                &wrong_context,
                &public_origin,
                credential_id,
                2_502,
            )
            .await
            .is_ok_and(|value| value.is_none())
    );
    let mut wrong_user = binding.clone();
    wrong_user.user_id = EntityId::new();
    let mut wrong_challenge = binding.clone();
    wrong_challenge.auth_challenge_id = EntityId::new();
    let mut wrong_session = binding.clone();
    wrong_session.session_id = Some(EntityId::new());
    let mut wrong_auth_revision = binding.clone();
    wrong_auth_revision.auth_revision = Revision::from_value(99);
    let wrong_rp_origin = WebAuthnOrigin::parse("https://other.example.test");
    assert!(wrong_rp_origin.is_ok());
    for candidate in [
        &wrong_user,
        &wrong_challenge,
        &wrong_session,
        &wrong_auth_revision,
    ] {
        assert!(
            database
                .webauthn_authentication_context(
                    auth_ceremony_id,
                    Revision::initial(),
                    candidate,
                    &public_origin,
                    credential_id,
                    2_502,
                )
                .await
                .is_ok_and(|value| value.is_none())
        );
    }
    if let Ok(wrong_rp_origin) = wrong_rp_origin {
        assert!(
            database
                .webauthn_authentication_context(
                    auth_ceremony_id,
                    Revision::initial(),
                    &binding,
                    &wrong_rp_origin,
                    credential_id,
                    2_502,
                )
                .await
                .is_ok_and(|value| value.is_none())
        );
    }
    let context = database
        .webauthn_authentication_context(
            auth_ceremony_id,
            Revision::initial(),
            &binding,
            &public_origin,
            credential_id,
            2_502,
        )
        .await;
    let (ceremony, stored_credential) = match context {
        Ok(Some(value)) => value,
        _ => panic!("exact authentication binding must load"),
    };
    let replacement_material = envelope(33);
    let first_commit = WebAuthnAuthenticationCommit {
        ceremony_id: ceremony.id,
        expected_ceremony_revision: ceremony.revision,
        binding: &binding,
        origin: &public_origin,
        credential_id: stored_credential.credential.id,
        expected_credential_revision: stored_credential.credential.revision,
        expected_sign_counter: 0,
        expected_backup_eligible: false,
        expected_backup_state: false,
        observed_sign_counter: 1,
        sign_counter: 1,
        backup_eligible: false,
        backup_state: false,
        backup_counter_anomaly: false,
        material: &replacement_material,
        now_ms: 2_503,
    };
    let second_commit = first_commit.clone();
    let mut forbidden_be_upgrade = first_commit.clone();
    forbidden_be_upgrade.backup_eligible = true;
    assert!(matches!(
        database
            .commit_webauthn_authentication(&forbidden_be_upgrade)
            .await,
        Err(PersistenceError::InvalidWebAuthnCredential)
    ));
    let mut forbidden_backup_state = first_commit.clone();
    forbidden_backup_state.backup_state = true;
    assert!(matches!(
        database
            .commit_webauthn_authentication(&forbidden_backup_state)
            .await,
        Err(PersistenceError::InvalidWebAuthnCredential)
    ));
    let (one, two) = tokio::join!(
        database.commit_webauthn_authentication(&first_commit),
        database.commit_webauthn_authentication(&second_commit),
    );
    assert!(matches!(
        (one, two),
        (
            Ok(WebAuthnAuthenticationCommitOutcome::Committed(_)),
            Ok(WebAuthnAuthenticationCommitOutcome::Stale)
        ) | (
            Ok(WebAuthnAuthenticationCommitOutcome::Stale),
            Ok(WebAuthnAuthenticationCommitOutcome::Committed(_))
        )
    ));
    assert!(matches!(
        database
            .commit_webauthn_authentication(&first_commit)
            .await,
        Ok(WebAuthnAuthenticationCommitOutcome::Stale)
    ));
    let success_rollback_at_ms = binding.reserved_at_ms;
    assert!(matches!(
        database
            .webauthn_authentication_handoff(
                auth_ceremony_id,
                Revision::initial(),
                &binding,
                &public_origin,
                success_rollback_at_ms,
            )
            .await,
        Ok(Some(WebAuthnAuthenticationHandoff::Verified))
    ));
    assert!(
        database
            .resume_auth_challenge_attempt(&AuthChallengeAttemptResume {
                access: authentication_access(&binding, 32, success_rollback_at_ms),
                claim_id: binding.claim_id,
                method: AuthenticationMethod::WebAuthn,
            })
            .await
            .is_ok_and(|value| value.is_some())
    );
    let success_recovery_at_ms = binding.verification_expires_at_ms + 1;
    for _ in 0..2 {
        assert!(matches!(
            database
                .webauthn_authentication_handoff(
                    auth_ceremony_id,
                    Revision::initial(),
                    &binding,
                    &public_origin,
                    success_recovery_at_ms,
                )
                .await,
            Ok(Some(WebAuthnAuthenticationHandoff::Verified))
        ));
    }
    let mut wrong_handoff_claim = binding.clone();
    wrong_handoff_claim.claim_id = EntityId::new();
    assert!(
        database
            .webauthn_authentication_handoff(
                auth_ceremony_id,
                Revision::initial(),
                &wrong_handoff_claim,
                &public_origin,
                success_recovery_at_ms,
            )
            .await
            .is_ok_and(|value| value.is_none())
    );
    let success_access = authentication_access(&binding, 32, success_recovery_at_ms);
    assert!(
        database
            .resume_auth_challenge_attempt(&AuthChallengeAttemptResume {
                access: success_access.clone(),
                claim_id: binding.claim_id,
                method: AuthenticationMethod::WebAuthn,
            })
            .await
            .is_ok_and(|value| value.is_some_and(|attempt| {
                attempt.reserved_at_ms == binding.reserved_at_ms
                    && attempt.verification_expires_at_ms == binding.verification_expires_at_ms
            }))
    );
    assert!(matches!(
        database
            .begin_auth_challenge_consumption(&AuthChallengeConsumption {
                access: success_access.clone(),
                claim_id: binding.claim_id,
                method: AuthenticationMethod::WebAuthn,
                achieved_assurance: AuthenticationAssurance::PhishingResistant,
                expected_revision: Revision::from_value(1),
            })
            .await,
        Ok(AuthChallengeConsumptionOutcome::Consumed(_))
    ));
    assert!(matches!(
        database
            .begin_auth_challenge_consumption(&AuthChallengeConsumption {
                access: success_access,
                claim_id: binding.claim_id,
                method: AuthenticationMethod::WebAuthn,
                achieved_assurance: AuthenticationAssurance::PhishingResistant,
                expected_revision: Revision::from_value(1),
            })
            .await,
        Ok(AuthChallengeConsumptionOutcome::Stale)
    ));

    let rejected_user = EntityId::new();
    let rejected_session = EntityId::new();
    let rejected_other = EntityId::new();
    insert_principal(
        &database,
        rejected_user,
        rejected_session,
        rejected_other,
        34,
        1_000,
    )
    .await;
    let _rejected_credential = register_one(
        &database,
        rejected_user,
        rejected_session,
        35,
        2_400,
        0,
        false,
    )
    .await;
    let (rejected_binding, rejected_ceremony_id) = begin_bound_authentication(
        &database,
        rejected_user,
        rejected_session,
        1,
        36,
        2_550,
    )
    .await;
    assert!(
        database
            .reject_webauthn_authentication(
                rejected_ceremony_id,
                Revision::initial(),
                &rejected_binding,
                &public_origin,
                2_551,
            )
            .await
            .is_ok_and(|value| value)
    );
    let rejection_rollback_at_ms = rejected_binding.reserved_at_ms;
    assert!(matches!(
        database
            .webauthn_authentication_handoff(
                rejected_ceremony_id,
                Revision::initial(),
                &rejected_binding,
                &public_origin,
                rejection_rollback_at_ms,
            )
            .await,
        Ok(Some(WebAuthnAuthenticationHandoff::Rejected))
    ));
    assert!(
        database
            .resume_auth_challenge_attempt(&AuthChallengeAttemptResume {
                access: authentication_access(
                    &rejected_binding,
                    36,
                    rejection_rollback_at_ms,
                ),
                claim_id: rejected_binding.claim_id,
                method: AuthenticationMethod::WebAuthn,
            })
            .await
            .is_ok_and(|value| value.is_some())
    );
    let rejection_recovery_at_ms = rejected_binding.verification_expires_at_ms + 1;
    for _ in 0..2 {
        assert!(matches!(
            database
                .webauthn_authentication_handoff(
                    rejected_ceremony_id,
                    Revision::initial(),
                    &rejected_binding,
                    &public_origin,
                    rejection_recovery_at_ms,
                )
                .await,
            Ok(Some(WebAuthnAuthenticationHandoff::Rejected))
        ));
    }
    let rejection_access =
        authentication_access(&rejected_binding, 36, rejection_recovery_at_ms);
    assert!(
        database
            .resume_auth_challenge_attempt(&AuthChallengeAttemptResume {
                access: rejection_access.clone(),
                claim_id: rejected_binding.claim_id,
                method: AuthenticationMethod::WebAuthn,
            })
            .await
            .is_ok_and(|value| value.is_some())
    );
    assert!(matches!(
        database
            .record_auth_challenge_failure(&AuthChallengeAttemptFailure {
                access: rejection_access.clone(),
                claim_id: rejected_binding.claim_id,
                method: AuthenticationMethod::WebAuthn,
                expected_revision: Revision::from_value(1),
            })
            .await,
        Ok(AuthChallengeAttemptOutcome::Retryable(_))
    ));
    assert!(matches!(
        database
            .record_auth_challenge_failure(&AuthChallengeAttemptFailure {
                access: rejection_access,
                claim_id: rejected_binding.claim_id,
                method: AuthenticationMethod::WebAuthn,
                expected_revision: Revision::from_value(1),
            })
            .await,
        Ok(AuthChallengeAttemptOutcome::Stale)
    ));

    // At the exact verifier-expiry boundary, C3 refresh and a method terminal commit race for
    // the same claim. The claim lock makes the result binary: either refresh wins and no
    // credential/ceremony mutation commits, or WebAuthn wins and refresh observes the terminal
    // pin instead of clearing the claim. There is no third half-committed state.
    let race_user = EntityId::new();
    let race_session = EntityId::new();
    let race_other = EntityId::new();
    insert_principal(
        &database,
        race_user,
        race_session,
        race_other,
        59,
        1_000,
    )
    .await;
    let race_credential =
        register_one(&database, race_user, race_session, 60, 5_000, 0, false).await;
    let (race_binding, race_ceremony_id) =
        begin_bound_authentication(&database, race_user, race_session, 1, 61, 5_200).await;
    let race_context = database
        .webauthn_authentication_context(
            race_ceremony_id,
            Revision::initial(),
            &race_binding,
            &public_origin,
            &race_credential.credential.credential_id,
            5_201,
        )
        .await;
    let (race_ceremony, race_stored) = match race_context {
        Ok(Some(value)) => value,
        _ => panic!("expiry-race authentication context must load"),
    };
    let race_material = envelope(62);
    let race_commit_at_ms = race_binding.verification_expires_at_ms - 1;
    let race_refresh_at_ms = race_binding.verification_expires_at_ms;
    let race_command = WebAuthnAuthenticationCommit {
        ceremony_id: race_ceremony.id,
        expected_ceremony_revision: race_ceremony.revision,
        binding: &race_binding,
        origin: &public_origin,
        credential_id: race_stored.credential.id,
        expected_credential_revision: race_stored.credential.revision,
        expected_sign_counter: 0,
        expected_backup_eligible: false,
        expected_backup_state: false,
        observed_sign_counter: 1,
        sign_counter: 1,
        backup_eligible: false,
        backup_state: false,
        backup_counter_anomaly: false,
        material: &race_material,
        now_ms: race_commit_at_ms,
    };
    let race_refresh_access = authentication_access(&race_binding, 61, race_refresh_at_ms);
    let (race_commit, race_refresh) = tokio::join!(
        database.commit_webauthn_authentication(&race_command),
        database.auth_challenge(&race_refresh_access),
    );
    let refreshed_challenge = match race_refresh {
        Ok(Some(value)) => value,
        _ => panic!("expiry race must retain a readable challenge projection"),
    };
    match race_commit {
        Ok(WebAuthnAuthenticationCommitOutcome::Committed(_)) => {
            assert_eq!(
                refreshed_challenge.status,
                AuthChallengeStatus::VerificationPending
            );
            assert_eq!(
                credential_counter(&database, race_credential.credential.id).await,
                1
            );
            assert!(matches!(
                database
                    .webauthn_authentication_handoff(
                        race_ceremony_id,
                        Revision::initial(),
                        &race_binding,
                        &public_origin,
                        race_refresh_at_ms + 1,
                    )
                    .await,
                Ok(Some(WebAuthnAuthenticationHandoff::Verified))
            ));
        }
        Ok(WebAuthnAuthenticationCommitOutcome::Stale) => {
            assert!(matches!(
                refreshed_challenge.status,
                AuthChallengeStatus::Pending | AuthChallengeStatus::Exhausted
            ));
            assert_eq!(
                credential_counter(&database, race_credential.credential.id).await,
                0
            );
            assert!(
                database
                    .webauthn_authentication_handoff(
                        race_ceremony_id,
                        Revision::initial(),
                        &race_binding,
                        &public_origin,
                        race_refresh_at_ms + 1,
                    )
                    .await
                    .is_ok_and(|value| value.is_none())
            );
        }
        Err(_) => panic!("expiry race commit must return a typed outcome"),
    }

    // Lazy authentication cleanup can happen after the exact expiry instant without violating
    // the terminal timestamp constraint: the terminal time is pinned to the stored expiry.
    let expiry_user = EntityId::new();
    let expiry_session = EntityId::new();
    let expiry_other = EntityId::new();
    insert_principal(
        &database,
        expiry_user,
        expiry_session,
        expiry_other,
        63,
        1_000,
    )
    .await;
    let expiry_credential =
        register_one(&database, expiry_user, expiry_session, 64, 5_400, 0, false).await;
    let (expiry_binding, expiry_ceremony_id) =
        begin_bound_authentication(&database, expiry_user, expiry_session, 1, 65, 5_600).await;
    let after_authentication_expiry = expiry_binding.verification_expires_at_ms + 1;
    assert!(
        database
            .webauthn_authentication_context(
                expiry_ceremony_id,
                Revision::initial(),
                &expiry_binding,
                &public_origin,
                &expiry_credential.credential.credential_id,
                after_authentication_expiry,
            )
            .await
            .is_ok_and(|value| value.is_none())
    );
    assert_eq!(
        ceremony_status_and_times(&database, expiry_ceremony_id).await,
        (
            "expired".to_owned(),
            expiry_binding.verification_expires_at_ms,
            Some(expiry_binding.verification_expires_at_ms),
        )
    );

    // Registration completion and credential management share the same principal/auth-state/
    // actor-session lock prefix. The race must finish without a lock timeout and advance the auth
    // revision exactly once; the loser observes a typed stale principal/transaction outcome.
    let management_race_user = EntityId::new();
    let management_race_session = EntityId::new();
    let management_race_other = EntityId::new();
    insert_principal(
        &database,
        management_race_user,
        management_race_session,
        management_race_other,
        68,
        1_000,
    )
    .await;
    let management_race_existing = register_one(
        &database,
        management_race_user,
        management_race_session,
        69,
        6_000,
        0,
        false,
    )
    .await;
    let management_race_ceremony = NewWebAuthnRegistrationCeremony {
        id: EntityId::new(),
        guard: guard(
            management_race_user,
            management_race_session,
            1,
            6_100,
            6_200,
        ),
        origin: public_origin.clone(),
        expires_at_ms: 106_200,
        state: envelope(70),
    };
    let management_race_stored = match database
        .begin_webauthn_registration(&management_race_ceremony)
        .await
    {
        Ok(BeginWebAuthnRegistrationOutcome::Created(stored)) => stored,
        _ => panic!("management race registration must begin"),
    };
    let management_race_new = credential(management_race_user, 71, 6_300, 0, false);
    let management_race_guard = guard(
        management_race_user,
        management_race_session,
        1,
        6_100,
        6_300,
    );
    let management_race_complete = CompleteWebAuthnRegistration {
        ceremony_id: management_race_stored.id,
        expected_ceremony_revision: management_race_stored.revision,
        guard: management_race_guard.clone(),
        origin: &public_origin,
        credential: &management_race_new,
    };
    let management_race_revoke = RevokeWebAuthnCredential {
        credential_id: management_race_existing.credential.id,
        expected_credential_revision: Revision::initial(),
        guard: management_race_guard,
    };
    let (management_complete, management_revoke) = tokio::join!(
        database.complete_webauthn_registration(&management_race_complete),
        database.revoke_webauthn_credential(&management_race_revoke),
    );
    let completion_won = matches!(
        &management_complete,
        Ok(CompleteWebAuthnRegistrationOutcome::Registered(_))
    );
    let revoke_won = matches!(
        &management_revoke,
        Ok(RevokeWebAuthnCredentialOutcome::Revoked { .. })
    );
    assert_ne!(completion_won, revoke_won);
    assert!(matches!(
        &management_complete,
        Ok(CompleteWebAuthnRegistrationOutcome::Registered(_))
            | Ok(CompleteWebAuthnRegistrationOutcome::Stale)
    ));
    assert!(matches!(
        &management_revoke,
        Ok(RevokeWebAuthnCredentialOutcome::Revoked { .. })
            | Ok(RevokeWebAuthnCredentialOutcome::Stale)
            | Err(PersistenceError::SessionPrincipalUnavailable)
    ));
    assert_eq!(auth_revision(&database, management_race_user).await, 2);

    // C3 creates a replacement challenge by locking stale challenge rows before its INSERT takes
    // principal FK KEY SHARE locks. Credential revocation takes the opposite logical prefix. Both
    // must complete without a PostgreSQL deadlock; revoke either invalidates the newly committed
    // challenge in its fresh statement snapshot or makes the create observe a stale auth revision.
    let create_revoke_user = EntityId::new();
    let create_revoke_session = EntityId::new();
    let create_revoke_other = EntityId::new();
    insert_principal(
        &database,
        create_revoke_user,
        create_revoke_session,
        create_revoke_other,
        72,
        1_000,
    )
    .await;
    let create_revoke_credential = register_one(
        &database,
        create_revoke_user,
        create_revoke_session,
        73,
        7_000,
        0,
        false,
    )
    .await;
    let stale_before_revoke = new_bound_challenge(
        create_revoke_user,
        create_revoke_session,
        1,
        AuthChallengePurpose::SensitiveAction,
        74,
        7_200,
        7_250,
    );
    assert!(matches!(
        database.create_auth_challenge(&stale_before_revoke).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    let replacement_during_revoke = new_bound_challenge(
        create_revoke_user,
        create_revoke_session,
        1,
        AuthChallengePurpose::SensitiveAction,
        75,
        7_300,
        7_400,
    );
    let revoke_during_create_command = RevokeWebAuthnCredential {
        credential_id: create_revoke_credential.credential.id,
        expected_credential_revision: Revision::initial(),
        guard: guard(
            create_revoke_user,
            create_revoke_session,
            1,
            7_100,
            7_300,
        ),
    };
    let (create_during_revoke, revoke_during_create) = tokio::join!(
        database.create_auth_challenge(&replacement_during_revoke),
        database.revoke_webauthn_credential(&revoke_during_create_command),
    );
    assert!(matches!(
        create_during_revoke,
        Ok(CreateAuthChallengeOutcome::Created(_))
            | Ok(CreateAuthChallengeOutcome::PrincipalUnavailable)
    ));
    assert!(matches!(
        revoke_during_create,
        Ok(RevokeWebAuthnCredentialOutcome::Revoked {
            auth_revision,
            ..
        }) if auth_revision == Revision::from_value(2)
    ));

    // Revocation must also tolerate a lazily stale pending ceremony. Expired rows are marked at
    // their stored expiry, while still-live rows are rejected at the management commit time.
    let stale_registration = NewWebAuthnRegistrationCeremony {
        id: EntityId::new(),
        guard: guard(user_id, actor_session_id, 1, 2_100, 2_550),
        origin: public_origin.clone(),
        expires_at_ms: 2_599,
        state: envelope(66),
    };
    assert!(matches!(
        database
            .begin_webauthn_registration(&stale_registration)
            .await,
        Ok(BeginWebAuthnRegistrationOutcome::Created(_))
    ));
    let (_live_binding, live_authentication_id) = begin_bound_authentication(
        &database,
        user_id,
        actor_session_id,
        1,
        67,
        2_580,
    )
    .await;
    let revoked = database
        .revoke_webauthn_credential(&RevokeWebAuthnCredential {
            credential_id: first_credential.credential.id,
            expected_credential_revision: Revision::from_value(2),
            guard: guard(user_id, actor_session_id, 1, 2_100, 2_600),
        })
        .await;
    assert!(matches!(
        revoked,
        Ok(RevokeWebAuthnCredentialOutcome::Revoked {
            auth_revision,
            ..
        }) if auth_revision == Revision::from_value(2)
    ));
    assert_eq!(auth_revision(&database, user_id).await, 2);
    assert_eq!(
        ceremony_status_and_times(&database, stale_registration.id).await,
        ("expired".to_owned(), 2_599, Some(2_599))
    );
    let (live_status, _, live_finished_at_ms) =
        ceremony_status_and_times(&database, live_authentication_id).await;
    assert_eq!(live_status, "rejected");
    assert_eq!(live_finished_at_ms, Some(2_600));

    let clone_user = EntityId::new();
    let clone_session = EntityId::new();
    let clone_other = EntityId::new();
    insert_principal(
        &database,
        clone_user,
        clone_session,
        clone_other,
        40,
        1_000,
    )
    .await;
    let clone_credential = register_one(
        &database,
        clone_user,
        clone_session,
        41,
        3_000,
        5,
        false,
    )
    .await;
    let stale_before_clone = new_bound_challenge(
        clone_user,
        clone_session,
        1,
        AuthChallengePurpose::SensitiveAction,
        44,
        3_101,
        3_150,
    );
    assert!(matches!(
        database.create_auth_challenge(&stale_before_clone).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    let (clone_binding, clone_ceremony_id) = begin_bound_authentication(
        &database,
        clone_user,
        clone_session,
        1,
        42,
        3_200,
    )
    .await;
    let clone_material = envelope(43);
    assert!(matches!(
        database
            .commit_webauthn_authentication(&WebAuthnAuthenticationCommit {
                ceremony_id: clone_ceremony_id,
                expected_ceremony_revision: Revision::initial(),
                binding: &clone_binding,
                origin: &public_origin,
                credential_id: clone_credential.credential.id,
                expected_credential_revision: Revision::initial(),
                expected_sign_counter: 5,
                expected_backup_eligible: false,
                expected_backup_state: false,
                observed_sign_counter: 4,
                sign_counter: 5,
                backup_eligible: false,
                backup_state: false,
                backup_counter_anomaly: false,
                material: &clone_material,
                now_ms: 3_201,
            })
            .await,
        Err(PersistenceError::InvalidWebAuthnCredential)
    ));
    let replacement_during_clone = new_bound_challenge(
        clone_user,
        clone_session,
        1,
        AuthChallengePurpose::SensitiveAction,
        45,
        3_201,
        3_301,
    );
    let clone_command = WebAuthnCloneSuspected {
        ceremony_id: clone_ceremony_id,
        expected_ceremony_revision: Revision::initial(),
        binding: &clone_binding,
        origin: &public_origin,
        credential_id: clone_credential.credential.id,
        expected_credential_revision: Revision::initial(),
        expected_sign_counter: 5,
        now_ms: 3_201,
    };
    let (create_during_clone, clone_outcome) = tokio::join!(
        database.create_auth_challenge(&replacement_during_clone),
        database.record_webauthn_clone_suspected(&clone_command),
    );
    assert!(matches!(
        create_during_clone,
        Ok(CreateAuthChallengeOutcome::Created(_))
            | Ok(CreateAuthChallengeOutcome::PrincipalUnavailable)
    ));
    assert!(matches!(
        clone_outcome,
        Ok(WebAuthnCloneSuspectedOutcome::Recorded {
            auth_revision,
            revoked_sessions: 1,
        }) if auth_revision == Revision::from_value(2)
    ));
    assert_eq!(session_status(&database, clone_session).await, "revoked");

    let synced_user = EntityId::new();
    let synced_session = EntityId::new();
    let synced_other = EntityId::new();
    insert_principal(
        &database,
        synced_user,
        synced_session,
        synced_other,
        50,
        1_000,
    )
    .await;
    let synced = register_one(
        &database,
        synced_user,
        synced_session,
        51,
        4_000,
        5,
        true,
    )
    .await;
    let (synced_binding, synced_ceremony_id) = begin_bound_authentication(
        &database,
        synced_user,
        synced_session,
        1,
        52,
        4_200,
    )
    .await;
    let synced_context = database
        .webauthn_authentication_context(
            synced_ceremony_id,
            Revision::initial(),
            &synced_binding,
            &public_origin,
            &synced.credential.credential_id,
            4_201,
        )
        .await;
    let (synced_ceremony, synced_stored) = match synced_context {
        Ok(Some(value)) => value,
        _ => panic!("synced credential context must load"),
    };
    let synced_material = envelope(53);
    let synced_command = WebAuthnAuthenticationCommit {
        ceremony_id: synced_ceremony.id,
        expected_ceremony_revision: synced_ceremony.revision,
        binding: &synced_binding,
        origin: &public_origin,
        credential_id: synced_stored.credential.id,
        expected_credential_revision: synced_stored.credential.revision,
        expected_sign_counter: 5,
        expected_backup_eligible: true,
        expected_backup_state: false,
        observed_sign_counter: 5,
        sign_counter: 5,
        backup_eligible: true,
        backup_state: true,
        backup_counter_anomaly: true,
        material: &synced_material,
        now_ms: 4_202,
    };
    let mut forbidden_be_downgrade = synced_command.clone();
    forbidden_be_downgrade.backup_eligible = false;
    forbidden_be_downgrade.backup_state = false;
    forbidden_be_downgrade.backup_counter_anomaly = false;
    assert!(matches!(
        database
            .commit_webauthn_authentication(&forbidden_be_downgrade)
            .await,
        Err(PersistenceError::InvalidWebAuthnCredential)
    ));
    let mut omitted_synced_anomaly = synced_command.clone();
    omitted_synced_anomaly.backup_counter_anomaly = false;
    assert!(matches!(
        database
            .commit_webauthn_authentication(&omitted_synced_anomaly)
            .await,
        Err(PersistenceError::InvalidWebAuthnCredential)
    ));
    let synced_commit = database
        .commit_webauthn_authentication(&synced_command)
        .await;
    assert!(matches!(
        synced_commit,
        Ok(WebAuthnAuthenticationCommitOutcome::Committed(ref value))
            if value.backup_eligible
                && value.backup_state
                && value.sign_counter == 5
                && value.backup_counter_anomaly_at_ms == Some(4_202)
    ));

    let invalid_uv = match &database {
        Database::Sqlite(pool) => sqlx::query(
            "INSERT INTO webauthn_credentials (id,user_id,credential_id,user_handle,aaguid,user_verified,backup_eligible,backup_state,sign_counter,nickname,status,material_schema_version,material_key_version,material_nonce,material_ciphertext,material_aad_hash,created_at_ms,last_used_at_ms,backup_counter_anomaly_at_ms,revoked_at_ms,clone_suspected_at_ms,revision) VALUES (?,?,?, ?,NULL,0,0,0,0,'bad','active',1,1,?,?,?,5000,NULL,NULL,NULL,NULL,0)",
        )
        .bind(EntityId::new().to_string())
        .bind(synced_user.to_string())
        .bind([88_u8; 32].as_slice())
        .bind(synced_user.into_uuid().as_bytes().as_slice())
        .bind([1_u8; 24].as_slice())
        .bind([2_u8; 32].as_slice())
        .bind([3_u8; 32].as_slice())
        .execute(pool)
        .await
        .is_err(),
        Database::Postgres(pool) => sqlx::query(
            "INSERT INTO webauthn_credentials (id,user_id,credential_id,user_handle,aaguid,user_verified,backup_eligible,backup_state,sign_counter,nickname,status,material_schema_version,material_key_version,material_nonce,material_ciphertext,material_aad_hash,created_at_ms,last_used_at_ms,backup_counter_anomaly_at_ms,revoked_at_ms,clone_suspected_at_ms,revision) VALUES ($1,$2,$3,$4,NULL,false,false,false,0,'bad','active',1,1,$5,$6,$7,5000,NULL,NULL,NULL,NULL,0)",
        )
        .bind(EntityId::new().into_uuid())
        .bind(synced_user.into_uuid())
        .bind([88_u8; 32].as_slice())
        .bind(synced_user.into_uuid().as_bytes().as_slice())
        .bind([1_u8; 24].as_slice())
        .bind([2_u8; 32].as_slice())
        .bind([3_u8; 32].as_slice())
        .execute(pool)
        .await
        .is_err(),
    };
    assert!(invalid_uv);

    let invalid_backup_state = match &database {
        Database::Sqlite(pool) => sqlx::query(
            "INSERT INTO webauthn_credentials (id,user_id,credential_id,user_handle,aaguid,user_verified,backup_eligible,backup_state,sign_counter,nickname,status,material_schema_version,material_key_version,material_nonce,material_ciphertext,material_aad_hash,created_at_ms,last_used_at_ms,backup_counter_anomaly_at_ms,revoked_at_ms,clone_suspected_at_ms,revision) VALUES (?,?,?,?,NULL,1,0,1,0,'bad-backup','active',1,1,?,?,?,5000,NULL,NULL,NULL,NULL,0)",
        )
        .bind(EntityId::new().to_string())
        .bind(synced_user.to_string())
        .bind([89_u8; 32].as_slice())
        .bind(synced_user.into_uuid().as_bytes().as_slice())
        .bind([1_u8; 24].as_slice())
        .bind([2_u8; 32].as_slice())
        .bind([3_u8; 32].as_slice())
        .execute(pool)
        .await
        .is_err(),
        Database::Postgres(pool) => sqlx::query(
            "INSERT INTO webauthn_credentials (id,user_id,credential_id,user_handle,aaguid,user_verified,backup_eligible,backup_state,sign_counter,nickname,status,material_schema_version,material_key_version,material_nonce,material_ciphertext,material_aad_hash,created_at_ms,last_used_at_ms,backup_counter_anomaly_at_ms,revoked_at_ms,clone_suspected_at_ms,revision) VALUES ($1,$2,$3,$4,NULL,true,false,true,0,'bad-backup','active',1,1,$5,$6,$7,5000,NULL,NULL,NULL,NULL,0)",
        )
        .bind(EntityId::new().into_uuid())
        .bind(synced_user.into_uuid())
        .bind([89_u8; 32].as_slice())
        .bind(synced_user.into_uuid().as_bytes().as_slice())
        .bind([1_u8; 24].as_slice())
        .bind([2_u8; 32].as_slice())
        .bind([3_u8; 32].as_slice())
        .execute(pool)
        .await
        .is_err(),
    };
    assert!(invalid_backup_state);

    assert!(!insert_raw_credential_id_length(&database, synced_user, 15, 90).await);
    assert!(insert_raw_credential_id_length(&database, synced_user, 16, 91).await);
    assert!(insert_raw_credential_id_length(&database, synced_user, 1_023, 92).await);
    assert!(!insert_raw_credential_id_length(&database, synced_user, 1_024, 93).await);
}

#[tokio::test]
async fn sqlite_webauthn_repository_contract() {
    let database = Database::connect("sqlite::memory:", settings()).await;
    assert!(database.is_ok());
    if let Ok(database) = database {
        repository_contract(database).await;
    }
}

struct PostgresFixture {
    database: Database,
    admin: PgPool,
}

async fn postgres_fixture(url: &str) -> Result<PostgresFixture, sqlx::Error> {
    let admin = PgPoolOptions::new().max_connections(1).connect(url).await?;
    sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_webauthn_c5 CASCADE")
        .execute(&admin)
        .await?;
    sqlx::query("CREATE SCHEMA nodecontroll_test_webauthn_c5")
        .execute(&admin)
        .await?;
    let options = PgConnectOptions::from_str(url)?.options([
        ("search_path", "nodecontroll_test_webauthn_c5"),
        ("statement_timeout", "30s"),
        ("lock_timeout", "5s"),
    ]);
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect_with(options)
        .await?;
    Ok(PostgresFixture {
        database: Database::Postgres(pool),
        admin,
    })
}

#[tokio::test]
async fn postgres_webauthn_repository_contract() {
    let url = match std::env::var("NODECONTROLL_TEST_POSTGRES_URL") {
        Ok(url) => url,
        Err(_) => panic!("NODECONTROLL_TEST_POSTGRES_URL is required for the persistence gate"),
    };
    let fixture = postgres_fixture(&url).await;
    assert!(fixture.is_ok());
    if let Ok(fixture) = fixture {
        repository_contract(fixture.database.clone()).await;
        if let Database::Postgres(pool) = &fixture.database {
            pool.close().await;
        }
        assert!(
            sqlx::query("DROP SCHEMA nodecontroll_test_webauthn_c5 CASCADE")
                .execute(&fixture.admin)
                .await
                .is_ok()
        );
        fixture.admin.close().await;
    }
}
