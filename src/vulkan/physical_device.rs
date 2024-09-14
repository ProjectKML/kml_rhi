use std::{ffi::CStr, sync::Arc};

use ash::vk;

struct Inner {
    physical_device: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties2<'static>,
}

#[derive(Clone)]
pub struct VulkanPhysicalDevice(Arc<Inner>);

impl VulkanPhysicalDevice {
    pub unsafe fn new(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        let mut properties = vk::PhysicalDeviceProperties2::default();

        instance.get_physical_device_properties2(physical_device, &mut properties);

        Self(Arc::new(Inner {
            physical_device,
            properties,
        }))
    }

    #[inline]
    pub fn physical_device(&self) -> &vk::PhysicalDevice {
        &self.0.physical_device
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.properties.properties.device_name.as_ptr())
                .to_str()
                .unwrap()
        }
    }
}
