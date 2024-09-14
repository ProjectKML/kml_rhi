mod device;
mod instance;
mod physical_device;

pub use device::*;
pub use instance::*;
pub use physical_device::*;
use thiserror::Error;

#[cfg(feature = "metal")]
use crate::metal::MetalError;
#[cfg(feature = "vulkan")]
use crate::vulkan::VulkanError;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "metal")]
    #[error("Metal backend: {0}")]
    MetalBackend(#[from] MetalError),
    #[cfg(feature = "vulkan")]
    #[error("Vulkan backend: {0}")]
    VulkanBackend(#[from] VulkanError),
}
