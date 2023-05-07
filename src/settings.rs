use config::{Config, ConfigError, Environment, File};
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use serde::Deserialize;
use std::str::FromStr;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub admin_token: String,
    pub secret: String,
    pub adr: String,
    pub port: u16,
    pub default_page_limit: u32,
    pub db_start_local: bool,
    pub db_cmd: String,
    pub db_adr: String,
    pub db_ns: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub log_level: String,
    pub debug: bool,
}

impl Settings {
    pub fn load() -> Self {
        match Settings::build() {
            Ok(settings) => settings,
            Err(why) => panic!("{}", why),
        }
    }

    fn build() -> Result<Settings, ConfigError> {
        let config = Config::builder()
            .set_default("adr", "0.0.0.0")?
            .set_default("port", 8000)?
            .set_default("default_page_limit", 8)?
            .set_default("db_cmd", "surreal")?
            .set_default("db_start_local", true)?
            .set_default("db_adr", "ws://localhost:8001")?
            .set_default("db_ns", "bgcat")?
            .set_default("db_name", "bgcat")?
            .set_default("db_user", "root")?
            .set_default("db_pass", "root")?
            .set_default("log_level", "info")?
            .set_default("debug", false)?
            .add_source(File::with_name("bgcat.toml").required(false))
            .add_source(Environment::with_prefix("BGCAT").try_parsing(true))
            .build()?;
        Ok(config.try_deserialize()?)
    }

    pub fn setup_logger(level: &str) {
        let colors = ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::Green);

        let logger = fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{} [{}] [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    colors.color(record.level()),
                    record.target(),
                    message
                ))
            })
            .level(LevelFilter::from_str(level).unwrap())
            .chain(std::io::stdout())
            .apply();

        match logger {
            Err(why) => panic!("Error setting up logger with: {}", why),
            Ok(()) => (),
        }
    }
}
