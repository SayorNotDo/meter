use serde::Deserialize;
use std::fs;

use crate::utils;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigJWT {
    pub private_access_key: PathBuf,
    pub public_access_key: PathBuf,
    pub private_refresh_key: PathBuf,
    pub public_refresh_key: PathBuf,
}

impl ConfigJWT {
    pub fn read_private_access_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(utils::dir::get_project_root()?.join(&self.private_access_key))
    }
    pub fn read_public_access_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(utils::dir::get_project_root()?.join(&self.public_access_key))
    }
    pub fn read_private_refresh_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(utils::dir::get_project_root()?.join(&self.private_refresh_key))
    }

    #[allow(dead_code)]
    pub fn read_public_refresh_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(utils::dir::get_project_root()?.join(&self.public_refresh_key))
    }
}
