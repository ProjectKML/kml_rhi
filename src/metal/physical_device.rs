use crate::{Error, InstanceDesc};
use ash::vk::PhysicalDeviceFeatures;

pub struct MetalPhysicalDevice {}

impl MetalPhysicalDevice {
    pub fn new() -> Result<Self, Error> {
        todo!()
    }
    pub fn get_name(&self) -> &str {
        todo!()
    }

    pub fn get_supported_features(&self) -> &PhysicalDeviceFeatures {
        todo!()
    }
}
