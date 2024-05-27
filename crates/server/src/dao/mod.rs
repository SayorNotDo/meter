use crate::errors::ResourceType;

pub mod base;
pub mod user;


pub trait Entity {
    const RESOURCE: ResourceType;
}