use figment::{
    providers::{Env, Format, Toml},
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
    pub db: DbConfig,
    pub cache: CacheConfig,
}

impl ServerConfig {
    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), self.port)
    }
}

#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct CacheConfig {
    pub host: String,
    pub port: u16,
    pub ttl: u64,
}

impl AppConfig {
    pub fn figment() -> Figment {
        Figment::new()
            .join(Toml::file("config.toml"))
            .join(Env::prefixed("APP_").split("_"))
    }

    pub fn from<T: Provider>(provider: T) -> Result<Self, figment::Error> {
        Figment::from(provider).extract()
    }
}
