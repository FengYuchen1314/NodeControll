use anyhow::{Context, Result};

fn main() -> Result<()> {
    let json = serde_json::to_string_pretty(&nodecontroll_api::openapi())
        .context("could not serialize OpenAPI document")?;
    println!("{json}");
    Ok(())
}
