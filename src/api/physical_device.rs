pub enum PhysicalDevice {}

pub struct PhysicalDeviceFeatures {}

impl PhysicalDevice {
    pub fn get_name(&self) -> &str {
        todo!()
    }

    pub fn get_supported_features(&self) -> &PhysicalDeviceFeatures {
        todo!()
    }
}
