use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Io(io::Error),
    Parse(toml::de::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Parse(err)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub user: User,
    pub database: DatabaseConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user: User::default(),
            database: DatabaseConfig::default(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "admin".to_string(),
            password: "admin".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            hostname: "localhost".to_string(),
            port: 5432,
            username: "yaus".to_string(),
            password: "password".to_string(),
            database: "yaus".to_string(),
        }
    }
}

pub fn read_config(file_path: &str) -> Result<Config> {
    // Create or read the file based on it's current existence
    let path = Path::new(file_path);
    match &path.exists() {
        true => {
            // Read the file
            let raw_config = fs::read_to_string(&path)?;
            debug!("Found existing config file at {file_path}");
            Ok::<Config, Error>(toml::from_str::<Config>(&raw_config)?)
        }
        false => {
            // Create the file and it's parent directories
            fs::create_dir_all(&path.parent().unwrap())?;
            let mut file = File::create(path)?;
            file.write_all(include_bytes!("default_config.toml"))?;
            info!("Created new config file at {file_path}");
            Ok(Config::default())
        }
    }
}

impl Config {
    /// Uses an existing configuration and scans the environment variables
    /// Overrides any values from the config with more important environment variables
    pub fn scan_env(&mut self) {
        // User configuration
        if let Ok(username) = env::var("YAUS_USERNAME") {
            debug!("Selected `YAUS_USERNAME` over value from config file");
            self.user.username = username
        }
        if let Ok(password) = env::var("YAUS_PASSWORD") {
            debug!("Selected `YAUS_PASSWORD` over value from config file");
            self.user.password = password
        }

        // Database configuration
        if let Ok(db_hostname) = env::var("YAUS_DB_HOSTNAME") {
            debug!("Selected `YAUS_DB_HOSTNAME` over value from config file");
            self.database.hostname = db_hostname
        }
        if let Ok(db_port) = env::var("YAUS_DB_PORT") {
            if let Ok(parsed_port) = db_port.parse::<u16>() {
                debug!("Selected `YAUS_DB_PORT` over value from config file");
                self.database.port = parsed_port;
            }
        }
        if let Ok(db_username) = env::var("YAUS_DB_USERNAME") {
            debug!("Selected `YAUS_DB_USERNAME` over value from config file");
            self.database.username = db_username
        }
        if let Ok(db_password) = env::var("YAUS_DB_PASSWORD") {
            debug!("Selected `YAUS_DB_PASSWORD` over value from config file");
            self.database.password = db_password
        }
        if let Ok(db_database) = env::var("YAUS_DB_DATABASE") {
            debug!("Selected `YAUS_DB_DATABASE` over value from config file");
            self.database.database = db_database
        }
    }
}
