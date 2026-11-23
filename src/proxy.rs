//! Setters write the disabled side first because on Windows the last write decides the active mode,
//! and read it first because macOS and Linux would otherwise overwrite its address with empty values.

use anyhow::{Context, Error, Result};
use std::str::FromStr;
use sysproxy::{Autoproxy, Sysproxy};

#[derive(Clone)]
pub struct Server {
    host: String,
    port: u16,
}

impl FromStr for Server {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (host, port) = s.rsplit_once(':').context("expected host:port")?;
        let port = port.parse().with_context(|| format!("invalid port `{port}`"))?;
        Ok(Self { host: host.into(), port })
    }
}

pub fn query() -> Result<()> {
    let Sysproxy { host, bypass, port, enable } = Sysproxy::get_system_proxy()?;
    let Autoproxy { url, enable: auto_enable } = Autoproxy::get_auto_proxy()?;
    let host = if host.contains(':') && !host.starts_with('[') { format!("[{host}]") } else { host };

    println!("Proxy:          {enable}");
    println!("AutoProxy:      {auto_enable}");
    println!("ProxyServer:    {host}:{port}");
    println!("ProxyBypass:    {bypass}");

    if !url.is_empty() {
        println!("AutoProxyUrl:   {url}");
    }

    Ok(())
}

pub fn set_direct() -> Result<()> {
    set_pac(String::new())
}

pub fn set_pac(url: String) -> Result<()> {
    let system = Sysproxy::get_system_proxy()?;
    Sysproxy { enable: false, ..system }.set_system_proxy()?;
    Autoproxy { url, enable: true }.set_auto_proxy()?;
    Ok(())
}

pub fn set_global(Server { host, port }: Server, bypass: String) -> Result<()> {
    let auto = Autoproxy::get_auto_proxy()?;
    Autoproxy { enable: false, ..auto }.set_auto_proxy()?;
    Sysproxy { host, bypass, port, enable: true }.set_system_proxy()?;
    Ok(())
}
