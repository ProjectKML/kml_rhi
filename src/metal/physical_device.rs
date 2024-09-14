use std::sync::Arc;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLDevice;

use crate::{Error, PhysicalDeviceFeatures};

struct Inner {
    mtl_device: Retained<ProtocolObject<dyn MTLDevice>>,
    name: String,
}

#[derive(Clone)]
pub struct MetalPhysicalDevice(Arc<Inner>);

impl MetalPhysicalDevice {
    pub fn new(mtl_device: Retained<ProtocolObject<dyn MTLDevice>>) -> Result<Self, Error> {
        let name = mtl_device.name().to_string();

        Ok(Self(Arc::new(Inner { mtl_device, name })))
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        &self.0.name
    }

    pub fn get_supported_features(&self) -> &PhysicalDeviceFeatures {
        todo!()
    }
}
