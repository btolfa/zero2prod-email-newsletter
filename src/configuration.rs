use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let username = &self.username;
        let password = &self.password;
        let port = self.port;
        let host = &self.host;
        let database_name = &self.database_name;

        format!("postgres://{username}:{password}@{host}:{port}/{database_name}")
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let s = Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?;
    s.try_deserialize()
}
