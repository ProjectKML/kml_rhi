use bitflags::bitflags;

#[cfg(feature = "metal")]
use crate::metal::MetalInstance;
#[cfg(feature = "vulkan")]
use crate::vulkan::VulkanInstance;
use crate::Error;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct InstanceFlags : u32 {
        const ENABLE_VALIDATION = 1 << 0;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BackendType {
    #[cfg(feature = "metal")]
    Metal,
    #[cfg(feature = "vulkan")]
    Vulkan,
}

impl Default for BackendType {
    fn default() -> Self {
        if cfg!(feature = "metal") {
            Self::Metal
        } else if cfg!(all(feature = "vulkan", not(feature = "metal"))) {
            Self::Vulkan
        } else {
            unimplemented!("Backend not implemented")
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct InstanceDesc {
    pub flags: InstanceFlags,
    pub backend_type: BackendType,
}

pub enum Instance {
    #[cfg(feature = "metal")]
    Metal(MetalInstance),
    #[cfg(feature = "vulkan")]
    Vulkan(VulkanInstance),
}

impl Instance {
    pub fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        match desc.backend_type {
            #[cfg(feature = "metal")]
            BackendType::Metal => Ok(Self::Metal(MetalInstance::new(desc)?)),
            #[cfg(feature = "vulkan")]
            BackendType::Vulkan => Ok(Self::Vulkan(VulkanInstance::new(desc)?)),
        }
    }
}