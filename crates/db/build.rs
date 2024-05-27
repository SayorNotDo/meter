use std::env;
use std::fs;

use serde::Deserialize;

use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    storage: ConfigStorage,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigStorage {
    database_url: String,
}

impl Config {
    fn parse(path: &str) -> anyhow::Result<Self> {
        let config_str = fs::read_to_string(path)?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }
}

fn main() {
    /* Compile SQL File */
    cornucopia()
}

fn cornucopia() {
    /* configuration file parse */
    let config = Config::parse("./../../config.toml").expect("Failed to parse configuration file");

    println!(
        "cargo:rustc-env=DATABASE_URL={}",
        &config.storage.database_url
    );

    let queries_path = "queries";

    let out_dir = env::var("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        // .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(&config.storage.database_url)
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }
}
