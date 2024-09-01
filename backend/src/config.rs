use figment::{
    providers::{Format, Toml},
    Figment, Provider,
};
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl ServerConfig {
    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), self.port)
    }
}

impl AppConfig {
    pub fn figment() -> Figment {
        Figment::new().join(Toml::file("config.toml"))
    }

    pub fn from<T: Provider>(provider: T) -> Result<Self, figment::Error> {
        Figment::from(provider).extract()
    }
}
