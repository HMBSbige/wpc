use crate::proxy::ProxyService;
use anyhow::Result;

pub fn execute() -> Result<()> {
    ProxyService::set_direct()?;
    println!("success");
    Ok(())
}
