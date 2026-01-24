use crate::proxy::ProxyService;
use anyhow::Result;

pub fn execute(url: &str, bypass: Option<&str>) -> Result<()> {
    ProxyService::set_global(url, bypass.unwrap_or_default())?;
    println!("success");
    Ok(())
}
