use crate::errors::ResourceType;

pub mod case;
pub mod file;
pub mod permission;
pub mod project;
pub mod user;

pub trait AppEntity {
    const RESOURCE: ResourceType;
}
