use config::ConfigError;

use crate::utils::dir::get_project_root;

pub mod template;
pub mod tracing;

pub fn get_static_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|err| ConfigError::Message(err.to_string()))?
        .join("static"))
}
