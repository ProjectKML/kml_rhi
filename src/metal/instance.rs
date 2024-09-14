use crate::metal::MetalPhysicalDevice;
use crate::{Error, InstanceDesc, PhysicalDevice};
use objc2::__framework_prelude::Retained;
use objc2_metal::{MTLCopyAllDevices, MTLDevice};

pub struct MetalInstance {
    physical_devices: Vec<PhysicalDevice>,
}

impl MetalInstance {
    pub fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        let devices = {
            let ptr = unsafe { MTLCopyAllDevices().as_ptr() };
            unsafe { Retained::retain(ptr) }.ok_or(Error::MetalBackend(String::from(
                "Failed to get metal devices",
            )))
        }?;

        let physical_devices = devices
            .iter()
            .map(|device| Ok(PhysicalDevice::Metal(MetalPhysicalDevice::new()?)))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Self { physical_devices })
    }
}
