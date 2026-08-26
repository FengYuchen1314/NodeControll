use tracing_subscriber::{
    Layer,
    filter::{EnvFilter, FilterExt, filter_fn},
    layer::SubscriberExt,
};

const OWNED_TARGETS: [&str; 10] = [
    "nodecontroll_agent",
    "nodecontroll_api",
    "nodecontroll_application",
    "nodecontroll_config",
    "nodecontroll_domain",
    "nodecontroll_identity",
    "nodecontroll_master",
    "nodecontroll_object_store",
    "nodecontroll_persistence",
    "nodecontroll_secrets",
];

pub(crate) fn init() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let environment =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let immutable_allowlist = filter_fn(|metadata| target_is_owned(metadata.target()));
    let format_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_filter(immutable_allowlist.and(environment));
    let subscriber = tracing_subscriber::registry().with(format_layer);

    // Fail closed if any caller installed another subscriber first. There is exactly one output
    // layer and tracing-subscriber's optional log compatibility bridge is disabled in Cargo.toml.
    tracing::subscriber::set_global_default(subscriber)
}

fn target_is_owned(target: &str) -> bool {
    OWNED_TARGETS.iter().any(|owned| {
        target == *owned
            || target
                .strip_prefix(owned)
                .is_some_and(|suffix| suffix.starts_with("::"))
    })
}

#[cfg(test)]
mod tests {
    use std::{
        io::{self, Write},
        sync::{Arc, Mutex},
    };

    use tracing_subscriber::{
        Layer,
        filter::{EnvFilter, FilterExt, filter_fn},
        fmt::MakeWriter,
        layer::SubscriberExt,
    };

    use super::target_is_owned;

    #[derive(Clone, Default)]
    struct CapturedWriter {
        bytes: Arc<Mutex<Vec<u8>>>,
    }

    impl CapturedWriter {
        fn output(&self) -> String {
            match self.bytes.lock() {
                Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                Err(_) => String::new(),
            }
        }
    }

    impl Write for CapturedWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            let mut bytes = self
                .bytes
                .lock()
                .map_err(|_| io::Error::other("captured telemetry lock poisoned"))?;
            bytes.extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl<'writer> MakeWriter<'writer> for CapturedWriter {
        type Writer = Self;

        fn make_writer(&'writer self) -> Self::Writer {
            self.clone()
        }
    }

    #[test]
    fn malicious_environment_directives_cannot_enable_dependency_or_secret_events() {
        let writer = CapturedWriter::default();
        let environment = EnvFilter::new(
            "nodecontroll_master=trace,webauthn_rs=trace,webauthn_rs_core=trace,webauthn_rs_core::core=trace,webauthn_rs_proto=trace",
        );
        let immutable_allowlist = filter_fn(|metadata| target_is_owned(metadata.target()));
        let layer = tracing_subscriber::fmt::layer()
            .without_time()
            .with_ansi(false)
            .with_writer(writer.clone())
            .with_filter(immutable_allowlist.and(environment));
        let subscriber = tracing_subscriber::registry().with(layer);

        tracing::subscriber::with_default(subscriber, || {
            tracing::error!(
                target: "webauthn_rs",
                attestation_object = "attestation-secret-top",
                "external-top"
            );
            tracing::error!(
                target: "webauthn_rs_core::core",
                client_data_json = "client-data-secret-child",
                "external-core-child"
            );
            tracing::error!(
                target: "webauthn_rs_proto::authenticator",
                authenticator_data = "authenticator-data-secret-child",
                "external-proto-child"
            );
            tracing::trace!(
                target: "nodecontroll_master::telemetry_contract",
                reason = "library_rejected",
                "owned-stable-event"
            );
        });

        let output = writer.output();
        assert!(output.contains("owned-stable-event"));
        assert!(output.contains("library_rejected"));
        for forbidden in [
            "external-top",
            "external-core-child",
            "external-proto-child",
            "attestation-secret-top",
            "client-data-secret-child",
            "authenticator-data-secret-child",
        ] {
            assert!(!output.contains(forbidden));
        }
    }

    #[test]
    fn allowlist_uses_exact_crate_boundaries() {
        assert!(target_is_owned("nodecontroll_application"));
        assert!(target_is_owned("nodecontroll_application::webauthn"));
        assert!(!target_is_owned("nodecontroll_application_evil"));
        assert!(!target_is_owned("webauthn_rs"));
        assert!(!target_is_owned("webauthn_rs_core::core"));
        assert!(!target_is_owned("webauthn_rs_proto::authenticator"));
    }

    #[test]
    fn binaries_have_one_hardened_entrypoint_and_no_second_subscriber_or_log_bridge() {
        let master = include_str!("main.rs");
        let agent = include_str!("../../agent/src/main.rs");
        let openapi_export = include_str!("bin/export_openapi.rs");
        assert_eq!(master.matches("telemetry::init()").count(), 1);
        for source in [master, agent, openapi_export] {
            assert!(!source.contains("tracing_subscriber"));
            assert!(!source.contains("LogTracer"));
            assert!(!source.contains("tracing_log"));
        }
    }

    #[test]
    fn webauthn_source_drops_library_errors_before_stable_audit() {
        let source = include_str!("../../../crates/application/src/webauthn.rs");
        assert!(source.contains("NoSubscriber::new()"));
        assert!(source.contains("drop(library_error)"));
        assert_eq!(source.matches("tracing::warn!").count(), 1);
        assert!(!source.contains("error = ?"));
        assert!(!source.contains("source = ?"));
        assert!(!source.contains("response = ?"));
    }
}
