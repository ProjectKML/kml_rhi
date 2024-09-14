use std::ffi::CStr;

use ash::vk;

pub struct VulkanPhysicalDevice {
    physical_device: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties2<'static>,
}

impl VulkanPhysicalDevice {
    pub unsafe fn new(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        let mut properties = vk::PhysicalDeviceProperties2::default();

        instance.get_physical_device_properties2(physical_device, &mut properties);

        Self {
            physical_device,
            properties,
        }
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.properties.properties.device_name.as_ptr())
                .to_str()
                .unwrap()
        }
    }
}
