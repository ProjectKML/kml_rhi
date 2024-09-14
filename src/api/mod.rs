mod instance;
mod physical_device;

pub use instance::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "metal")]
    #[error("Metal backend: {0}")]
    MetalBackend(String),
    #[cfg(feature = "vulkan")]
    #[error("Vulkan backend: {0}")]
    VulkanBackend(#[from] ash::vk::Result),
}
