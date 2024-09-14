use ash::{vk, Entry};

use crate::{Error, InstanceDesc};

pub struct VulkanInstance {
    entry: Entry,
    instance: ash::Instance,
}

impl VulkanInstance {
    pub unsafe fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        let entry_loader = Entry::load().unwrap();

        let application_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);

        let instance_create_info =
            vk::InstanceCreateInfo::default().application_info(&application_info);

        let instance = entry_loader.create_instance(&instance_create_info, None)?;

        todo!()
    }
}
