use anyhow::{Context, Result};
use sysproxy::{Autoproxy, Sysproxy};

pub struct ProxyService;

impl ProxyService {
    pub fn query() -> Result<(Sysproxy, Autoproxy)> {
        let system_proxy = Sysproxy::get_system_proxy()?;
        let auto_proxy = Autoproxy::get_auto_proxy()?;

        Ok((system_proxy, auto_proxy))
    }

    pub fn set_direct() -> Result<()> {
        let mut system_proxy = Sysproxy::get_system_proxy()?;
        let mut auto_proxy = Autoproxy::get_auto_proxy()?;

        system_proxy.enable = false;
        auto_proxy.enable = true;
        auto_proxy.url = String::default();

        system_proxy.set_system_proxy()?;
        auto_proxy.set_auto_proxy()?;

        Ok(())
    }

    pub fn set_pac(url: &str) -> Result<()> {
        let mut system_proxy = Sysproxy::get_system_proxy()?;
        let mut auto_proxy = Autoproxy::get_auto_proxy()?;

        system_proxy.enable = false;
        auto_proxy.enable = true;
        auto_proxy.url = url.to_string();

        system_proxy.set_system_proxy()?;
        auto_proxy.set_auto_proxy()?;

        Ok(())
    }

    pub fn set_global(url: &str, bypass: &str) -> Result<()> {
        let (host, port) = url.split_once(':').context("Invalid url format, expected host:port")?;

        let mut auto_proxy = Autoproxy::get_auto_proxy()?;
        let mut system_proxy = Sysproxy::get_system_proxy()?;

        auto_proxy.enable = false;
        system_proxy.enable = true;
        system_proxy.host = host.to_string();
        system_proxy.port = port.parse()?;
        system_proxy.bypass = bypass.to_string();

        auto_proxy.set_auto_proxy()?;
        system_proxy.set_system_proxy()?;

        Ok(())
    }
}
