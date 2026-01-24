use crate::proxy::ProxyService;
use anyhow::Result;

pub fn execute() -> Result<()> {
    let (system_proxy, auto_proxy) = ProxyService::query()?;

    println!("Proxy:          {}", system_proxy.enable);
    println!("AutoProxy:      {}", auto_proxy.enable);
    println!("ProxyServer:    {}:{}", system_proxy.host, system_proxy.port);
    println!("ProxyBypass:    {}", system_proxy.bypass);

    if !auto_proxy.url.is_empty() {
        println!("AutoProxyUrl:   {}", auto_proxy.url);
    }

    Ok(())
}
