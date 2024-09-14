use objc2::rc::Retained;
use objc2_metal::{MTLCopyAllDevices, MTLDevice};

use crate::{metal::MetalPhysicalDevice, Error, InstanceDesc, PhysicalDevice};

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
            .into_iter()
            .map(|device| Ok(PhysicalDevice::Metal(MetalPhysicalDevice::new(device)?)))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Self { physical_devices })
    }

    pub fn get_physical_devices(&self) -> &[PhysicalDevice] {
        &self.physical_devices
    }
}
