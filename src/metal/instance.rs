use std::sync::Arc;

use objc2::rc::Retained;
use objc2_metal::{MTLCopyAllDevices, MTLDevice};

use crate::{
    metal::{MetalError, MetalPhysicalDevice},
    Error, InstanceDesc, PhysicalDevice,
};

struct Inner {
    physical_devices: Vec<PhysicalDevice>,
}

#[derive(Clone)]
pub struct MetalInstance(Arc<Inner>);

impl MetalInstance {
    pub fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        let devices = {
            let ptr = unsafe { MTLCopyAllDevices().as_ptr() };
            unsafe { Retained::retain(ptr) }.ok_or(Error::MetalBackend(MetalError::Custom(
                String::from("Failed to get metal devices"),
            )))
        }?;

        let physical_devices = devices
            .into_iter()
            .map(|device| Ok(PhysicalDevice::Metal(MetalPhysicalDevice::new(device)?)))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Self(Arc::new(Inner { physical_devices })))
    }

    #[inline]
    pub fn get_physical_devices(&self) -> &[PhysicalDevice] {
        &self.0.physical_devices
    }
}
