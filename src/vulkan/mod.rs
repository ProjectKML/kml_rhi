mod instance;
mod physical_device;

use std::str::Utf8Error;

use ash::vk;
pub use instance::*;
pub use physical_device::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VulkanError {
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    InvalidUtf8(#[from] Utf8Error),
    #[error("Error: {0}")]
    Error(#[from] vk::Result),
}
