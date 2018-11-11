extern crate toml;
extern crate app_dirs;

use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use self::app_dirs::{AppInfo, app_root};

const APP_INFO: AppInfo = AppInfo{name: "muni-schedule", author: "Bo Sorensen"};
const CONFIG_PATH : &str = "config.toml";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AppConfig {
    pub map: AppConfigMap,
    pub line_left: AppConfigLine,
    pub line_right: AppConfigLine,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AppConfigLine {
    pub tag: String,
    pub color: String,
    pub stops: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AppConfigMap {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: f64,
}

#[derive(Debug)]
pub enum AppConfigError {
    IoError(io::Error),
    TomlDeError(toml::de::Error),
    TomlSerError(toml::ser::Error),
    AppDirsError(app_dirs::AppDirsError),
}

impl From<app_dirs::AppDirsError> for AppConfigError {
    fn from(err: app_dirs::AppDirsError) -> AppConfigError {
        AppConfigError::AppDirsError(err)
    }
}

impl From<io::Error> for AppConfigError {
    fn from(err: io::Error) -> AppConfigError {
        AppConfigError::IoError(err)
    }
}

impl From<toml::de::Error> for AppConfigError {
    fn from(err: toml::de::Error) -> AppConfigError {
        AppConfigError::TomlDeError(err)
    }
}

impl From<toml::ser::Error> for AppConfigError {
    fn from(err: toml::ser::Error) -> AppConfigError {
        AppConfigError::TomlSerError(err)
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, AppConfigError> {
        let conf_path = config_path();
        println!("Loading config file from: {:?}", conf_path);
        let file = File::open(conf_path?)?;

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let app_config : AppConfig = toml::from_str(&contents)?;
        Ok(app_config)
    }
    
    pub fn save(&self) -> Result<(), AppConfigError> {
        let conf_path = config_path();
        println!("Writing config file to: {:?}", conf_path);
        let toml_str = toml::to_string(self)?;
        let mut file = File::create(&conf_path?)?;
        file.write_all(&toml_str.into_bytes())?;
        Ok(())
    }

    pub fn load_or_store_default() -> Self {
        let maybe_loaded_config = Self::load();
        match maybe_loaded_config {
            Ok(val) => val,
            Err(_err) => {
                let conf = Self::default_config();
                match conf.save() {
                    Ok(_) => { println!("Saved default config") },
                    Err(_) => { println!("Could not save default config to file!") },
                }
                conf
            }
        }
    }

    fn default_config() -> Self {
        AppConfig {
            map: AppConfigMap {
                latitude: 37.775483,
                longitude: -122.418777,
                zoom: 13.0,
            },
            line_left: AppConfigLine {
                tag: "N".to_string(),
                color: "729fcf".to_string(),
                stops: vec!("5419".to_string(), "6996".to_string()),
            },
            line_right: AppConfigLine {
                tag: "49".to_string(),
                color: "ef2929".to_string(),
                stops: vec!("6817".to_string(), "6821".to_string()),
            },
        }
    }
}

fn config_path() -> Result<PathBuf, AppConfigError> {
    let path = app_root(app_dirs::AppDataType::UserConfig, &APP_INFO)?.join(CONFIG_PATH);
    Ok(path)
}