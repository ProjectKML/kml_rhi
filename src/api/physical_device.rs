#[cfg(feature = "metal")]
use crate::metal::MetalPhysicalDevice;

pub enum PhysicalDevice {
    Metal(MetalPhysicalDevice),
}

pub struct PhysicalDeviceFeatures {}

impl PhysicalDevice {
    pub fn get_name(&self) -> &str {
        match self {
            PhysicalDevice::Metal(physical_device) => {
                physical_device.get_name()
            }
        }
    }

    pub fn get_supported_features(&self) -> &PhysicalDeviceFeatures {
        todo!()
    }
}
