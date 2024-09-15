use bitflags::bitflags;

#[cfg(feature = "metal")]
use crate::metal::MetalInstance;
#[cfg(feature = "vulkan")]
use crate::vulkan::VulkanInstance;
use crate::{
    api::physical_device::PhysicalDevice, vulkan::VulkanDevice, Device, DeviceDesc, Error,
};
use crate::metal::MetalDevice;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct InstanceFlags : u32 {
        const ENABLE_VALIDATION = 1 << 0;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BackendType {
    Metal,
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
    #[inline]
    pub unsafe fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        match desc.backend_type {
            #[cfg(feature = "metal")]
            BackendType::Metal => Ok(Self::Metal(MetalInstance::new(desc)?)),
            #[cfg(feature = "vulkan")]
            BackendType::Vulkan => Ok(Self::Vulkan(VulkanInstance::new(desc)?)),
            _ => todo!(),
        }
    }

    #[inline]
    pub fn get_physical_devices(&self) -> &[PhysicalDevice] {
        match self {
            #[cfg(feature = "metal")]
            Instance::Metal(instance) => instance.get_physical_devices(),
            #[cfg(feature = "vulkan")]
            Instance::Vulkan(instance) => instance.get_physical_devices(),
        }
    }

    #[inline]
    pub fn create_device(&self, desc: &DeviceDesc) -> Result<Device, Error> {
        match self {
            #[cfg(feature = "metal")]
            Instance::Metal(instance) => Ok(Device::Metal(MetalDevice::new(instance, desc)?)),
            #[cfg(feature = "vulkan")]
            Instance::Vulkan(instance) => Ok(Device::Vulkan(VulkanDevice::new(instance, desc)?)),
        }
    }
}
