use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLDevice;

use crate::{Error, PhysicalDeviceFeatures};

pub struct MetalPhysicalDevice {
    mtl_device: Retained<ProtocolObject<dyn MTLDevice>>,
    name: String,
}

impl MetalPhysicalDevice {
    pub fn new(mtl_device: Retained<ProtocolObject<dyn MTLDevice>>) -> Result<Self, Error> {
        let name = mtl_device.name().to_string();

        Ok(Self { mtl_device, name })
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_supported_features(&self) -> &PhysicalDeviceFeatures {
        todo!()
    }
}
