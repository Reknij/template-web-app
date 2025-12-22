use default_functions::*;
use serde::Deserialize;
use std::default::Default;

mod default_functions;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub security: SecurityConfig,

    #[serde(default)]
    pub db: DbConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct ServerConfig {
    #[serde(default = "default_server_host")]
    pub host: String,

    #[serde(default = "default_server_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct SecurityConfig {
    #[serde(default = "default_security_auth_key")]
    pub auth_key: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct DbConfig {
    #[serde(default = "default_db_url")]
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig::default(),
            security: SecurityConfig::default(),
            db: DbConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: default_server_host(),
            port: default_server_port(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig {
            auth_key: default_security_auth_key(),
        }
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig { url: default_db_url() }
    }
}

impl Config {
    pub fn load_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, crate::error::CommonError> {
        toml::from_str(&std::fs::read_to_string(path)?).map_err(|e| crate::error::CommonError::InvalidInput { message: e.to_string().into() })
    }
}
