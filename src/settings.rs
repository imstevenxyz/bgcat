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
    pub data_dir: String,
    pub webp_quality: f32,
    pub default_page_limit: u32,
    pub ui_page_limit_step: u32,
    pub db_adr: String,
    pub db_ns: String,
    pub db_name: String,
    pub db_user: Option<String>,
    pub db_pass: Option<String>,
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
            .set_default("data_dir", "./data")?
            .set_default("webp_quality", 1.00)?
            .set_default("default_page_limit", 8)?
            .set_default("ui_page_limit_step", 4)?
            .set_default("db_adr", "rocksdb:./data/bgcat.db")?
            .set_default("db_ns", "bgcat")?
            .set_default("db_name", "bgcat")?
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
