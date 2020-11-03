use std::net::SocketAddr;
use uclicious::{Uclicious, ObjectError};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::convert::{From, TryFrom};
use uclicious::{DEFAULT_DUPLICATE_STRATEGY, Priority};

#[derive(Debug, Uclicious, Clone)]
pub struct Config {
    pub database_path: String,
    pub listen: SocketAddr,
    pub mqtt: Mqtt,
    pub zwave: ZWave,
}

#[derive(Debug, Uclicious, Clone)]
#[ucl(skip_builder)]
pub struct Mqtt {
    pub host: SocketAddr,
    pub authentication: Option<MqttAuthentication>,
}

#[derive(Debug, Uclicious, Clone)]
#[ucl(skip_builder)]
pub struct MqttAuthentication {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Uclicious, Clone)]
#[ucl(skip_builder)]
pub struct ZWave {
    pub host: String,
    pub home_id: String,
    pub devices: Vec<Device>
}

#[derive(Debug, Uclicious, Clone)]
#[ucl(skip_builder)]
pub struct Device {
    pub id: String,
    pub device_type: String,
    #[ucl(try_from="String")]
    pub handler: DeviceType
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DeviceType {
    BooleanSwitch,
    DoubleSwitch
}

impl TryFrom<String> for DeviceType {
    type Error = ObjectError;

    fn try_from(value: String) -> Result<DeviceType, Self::Error> {
        match value.to_lowercase().as_str() {
            "booleanswitch" => Ok(DeviceType::BooleanSwitch),
            "doubleswitch" => Ok(DeviceType::DoubleSwitch),
            _ => Err(ObjectError::other(format!("{} is not a supported value.", value)))
        }
    }
}

impl Config {
    pub fn ensure_paths(&self) -> Result<(), Box<dyn Error>> {
        {
            if let Some(path) = Path::new(&self.database_path).parent() {
                std::fs::create_dir_all(path)?
            }
        }
        Ok(())
    }
    pub fn database_url(&self) -> String {
        format!("sqlite:{}", self.database_path)
    }
}

impl ConfigBuilder {
    pub fn set_igor_variables(&mut self) {
        if let Some(path) = get_project_data_dir() {
            self.register_variable("XDG_DATA_DIR", path.as_path().to_string_lossy());
        }
    }
    pub fn init(mut self) -> Result<Config, Box<dyn Error>> {
        self.set_igor_variables();
        self.add_chunk_full(include_str!("../../default.conf"), Priority::from(0), DEFAULT_DUPLICATE_STRATEGY)?;
        let ret = self.build();
        ret
    }
}

pub fn get_project_data_dir() -> Option<PathBuf> {
    directories_next::ProjectDirs::from("com", "igor", "gateway")
        .map(|dirs| dirs.data_dir().to_path_buf())
}
