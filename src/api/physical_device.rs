#[cfg(feature = "metal")]
use crate::metal::MetalPhysicalDevice;
#[cfg(feature = "vulkan")]
use crate::vulkan::VulkanPhysicalDevice;

#[derive(Clone)]
pub enum PhysicalDevice {
    #[cfg(feature = "metal")]
    Metal(MetalPhysicalDevice),
    #[cfg(feature = "vulkan")]
    Vulkan(VulkanPhysicalDevice),
}

pub struct PhysicalDeviceFeatures {}

impl PhysicalDevice {
    #[inline]
    pub fn get_name(&self) -> &str {
        match self {
            #[cfg(feature = "metal")]
            PhysicalDevice::Metal(physical_device) => physical_device.get_name(),
            #[cfg(feature = "vulkan")]
            PhysicalDevice::Vulkan(physcial_device) => physcial_device.get_name(),
        }
    }

    pub fn get_supported_features(&self) -> &PhysicalDeviceFeatures {
        todo!()
    }
}
