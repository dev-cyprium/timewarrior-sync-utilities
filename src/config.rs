use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

use crate::relative_to_home;

const DEFAULT_CONFIG_LOCATION: &str = ".timewarrior-sync/config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl UserConfig {
    pub fn new() -> Self {
        return UserConfig {
            hostname: None,
            port: None,
            username: None,
            password: None,
        };
    }

    pub fn validate(&self) -> bool {
        return self.hostname.is_some()
            && self.port.is_some()
            && self.username.is_some()
            && self.password.is_some();
    }

    pub fn unwrap(&self) -> (String, u16, String, String) {
        return (
            self.hostname.clone().unwrap(),
            self.port.unwrap(),
            self.username.clone().unwrap(),
            self.password.clone().unwrap(),
        );
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: UserConfig =
            toml::from_str(&content).expect("Syntax error in configuration file");
        return Ok(config);
    }

    pub fn set_hostname(&mut self, hostname: String) -> &Self {
        self.hostname = Some(hostname);
        return self;
    }

    pub fn set_port(&mut self, port: u16) -> &Self {
        self.port = Some(port);
        return self;
    }

    pub fn set_username(&mut self, username: String) -> &Self {
        self.username = Some(username);
        return self;
    }

    pub fn set_password(&mut self, password: String) -> &Self {
        self.password = Some(password);
        return self;
    }
}

pub fn create_config_file<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    File::create(path)
}

pub fn fill_config_file(config_file: &mut File, config_data: &UserConfig) -> std::io::Result<()> {
    let data = toml::to_string(config_data).unwrap();
    config_file.write_all(data.as_bytes())?;
    Ok(())
}

///
/// Checks if the config file is present in the default location
/// or in the location specified by the TIMEW_SYNC_CONFIG environment variable.
///
/// # Returns
///
/// `true` if the config file is present, `false` otherwise.
///
/// # Example
///
/// ```
/// use sync_time::config::config_file_present;
///
/// if !config_file_present() {
///    println!("Config file not present!");
/// }
/// ```
pub fn config_file_present() -> bool {
    let config_path = config_file_path();
    return File::open(config_path).is_ok();
}

pub fn config_file_path() -> String {
    let config_path = match env::var("TIMEW_SYNC_CONFIG") {
        Ok(path) => path,
        Err(_) => DEFAULT_CONFIG_LOCATION.to_string(),
    };

    return relative_to_home(&config_path);
}
