use std::{
    env,
    fmt::Display,
    fs::File,
    io::{self, Write},
    path::Path,
};

use serde::Deserialize;
use tokio::fs;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IO(io::Error),
    Parse(toml::de::Error),
    HomeDir,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::IO(err) => format!("{err}"),
                Error::Parse(err) => format!("{err}"),
                Error::HomeDir =>
                    format!("Could not determine your home directory: does it exist?"),
            }
        )
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Parse(err)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub url: String,
    pub user: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "https://example.com".to_string(),
            user: "admin".to_string(),
            password: "admin".to_string(),
        }
    }
}

pub fn file_path() -> Result<String> {
    match env::var("HOME") {
        Ok(home) => {
            if let Ok(xdg_home) = env::var("XDG_CONFIG_HOME") {
                Ok(format!("{}/yaus/config.toml", xdg_home))
            } else {
                Ok(format!("{}/.config/yaus/config.toml", home))
            }
        }
        Err(_) => Err(Error::HomeDir),
    }
}

pub async fn read_config(file_path: &str) -> Result<Config> {
    // Create or read the file based on it's current existence
    let path = Path::new(file_path);
    match &path.exists() {
        true => {
            // Read the file
            let raw_config: String = fs::read_to_string(&path).await?;
            Ok(toml::from_str::<Config>(&raw_config)?)
        }
        false => {
            // Create the file and it's parent directories
            fs::create_dir_all(&path.parent().unwrap()).await?;
            let mut file = File::create(path)?;
            file.write_all(include_bytes!("default_config.toml"))?;
            println!("Created new config file (at `{file_path}`)");
            Ok(Config::default())
        }
    }
}
