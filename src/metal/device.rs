use std::sync::Arc;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::MTLDevice;
use crate::{DeviceDesc, PhysicalDevice};
use crate::metal::{MetalError, MetalInstance};

struct Inner {
    mtl_device: Retained<ProtocolObject<dyn MTLDevice>>
}

#[derive(Clone)]
pub struct MetalDevice(Arc<Inner>);

impl MetalDevice {
    pub fn new(instance: &MetalInstance, desc: &DeviceDesc) -> Result<Self, MetalError> {
        let PhysicalDevice::Metal(physical_device) = &desc.physical_device else {
            return Err(MetalError::Custom("Invalid physical device".to_owned()));
        };

        Ok(Self(Arc::new(Inner {
            mtl_device: physical_device.get_mtl_device(),
        })))
    }
}
