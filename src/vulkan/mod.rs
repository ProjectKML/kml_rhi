mod device;
mod instance;
mod physical_device;

use std::str::Utf8Error;

use ash::vk;
pub use device::*;
pub use instance::*;
pub use physical_device::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VulkanError {
    #[error("{0}")]
    Custom(String),
    #[error("Error: {0}")]
    Error(#[from] vk::Result),
    #[error("{0}")]
    InvalidUtf8(#[from] Utf8Error),
}
