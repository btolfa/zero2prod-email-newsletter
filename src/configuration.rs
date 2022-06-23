use config::{Config, File};
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        let username = &self.username;
        let password = self.password.expose_secret();
        let port = self.port;
        let host = &self.host;
        let database_name = &self.database_name;

        Secret::new(format!(
            "postgres://{username}:{password}@{host}:{port}/{database_name}"
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        let username = &self.username;
        let password = self.password.expose_secret();
        let port = self.port;
        let host = &self.host;

        Secret::new(format!("postgres://{username}:{password}@{host}:{port}"))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let s = Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?;
    s.try_deserialize()
}
