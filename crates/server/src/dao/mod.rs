use crate::errors::ResourceType;

mod base;
pub mod user;


pub trait Entity {
    const RESOURCE: ResourceType;
}