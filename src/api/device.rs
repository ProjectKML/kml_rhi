#[cfg(feature = "metal")]
use crate::metal::MetalDevice;
#[cfg(feature = "vulkan")]
use crate::vulkan::VulkanDevice;
use crate::PhysicalDevice;

#[derive(Clone)]
pub struct DeviceDesc {
    pub physical_device: PhysicalDevice,
}

#[derive(Clone)]
pub enum Device {
    #[cfg(feature = "metal")]
    Metal(MetalDevice),
    #[cfg(feature = "vulkan")]
    Vulkan(VulkanDevice),
}
