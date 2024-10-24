use config::{ConfigError, Environment};
use serde::Deserialize;
use std::{env, str::FromStr};

use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize)]
struct Config {
    storage: ConfigStorage,
}

#[derive(Debug, Clone, Deserialize)]
struct ConfigStorage {
    database_url: String,
}

impl Config {
    fn read(env: Environment) -> Result<Self, ConfigError> {
        let config_dir = get_setting_dir()?;
        let profile = std::env::var("APP_PROFILE")
            .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
            .unwrap_or_else(|_| Ok(Profile::Dev))?;
        let profile_filename = format!("{profile}.toml");
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(config::File::from(config_dir.join(profile_filename)))
            .add_source(env)
            .build()?;
        config.try_deserialize()
    }
}

fn get_setting_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("settings"))
}

fn get_project_root() -> std::io::Result<PathBuf> {
    if let Some(root) = get_cargo_project_root()? {
        Ok(root)
    } else {
        Ok(std::env::current_dir()?)
    }
}

fn get_cargo_project_root() -> std::io::Result<Option<PathBuf>> {
    let current_path = std::env::current_dir()?;
    for ancestor in current_path.ancestors() {
        for dir in std::fs::read_dir(ancestor)? {
            let dir = dir?;
            if dir.file_name() == "Cargo.lock" {
                return Ok(Some(ancestor.to_path_buf()));
            }
        }
    }
    Ok(None)
}

fn get_env_source(prefix: &str) -> config::Environment {
    config::Environment::with_prefix(prefix)
        .prefix_separator("__")
        .separator("__")
}

#[derive(
    Debug,
    Deserialize,
    strum::Display,
    strum::EnumString,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum Profile {
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}

fn main() {
    /* Compile SQL File */
    cornucopia()
}

fn cornucopia() {
    /* configuration file parse */
    let config =
        Config::read(get_env_source("APP".into())).expect("failed to parse configuration file.");

    println!(
        "cargo:rustc-env=DATABASE_URL={}",
        &config.storage.database_url
    );

    let queries_path = "queries";
    let migrations_path = "migrations";

    let out_dir = env::var("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={migrations_path}");
    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        // .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        // .arg("schema")
        // .arg(migrations_path)
        .arg("live")
        .arg(&config.storage.database_url)
        .output()
        .unwrap();

    if !output.status.success() {
        panic!(
            "Build Failure: {}",
            &std::str::from_utf8(&output.stderr).unwrap()
        );
    }
}
