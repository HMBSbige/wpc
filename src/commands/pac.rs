use crate::proxy::ProxyService;
use anyhow::Result;

pub fn execute(url: &str) -> Result<()> {
    ProxyService::set_pac(url)?;
    println!("success");
    Ok(())
}
