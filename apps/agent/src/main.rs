use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
struct AgentBootstrapInfo {
    product: &'static str,
    version: &'static str,
    protocol_status: &'static str,
}

fn main() -> Result<()> {
    let info = AgentBootstrapInfo {
        product: "NodeControll Agent",
        version: env!("CARGO_PKG_VERSION"),
        protocol_status: "skeleton-not-enrolled",
    };
    println!("{}", serde_json::to_string(&info)?);
    Ok(())
}
