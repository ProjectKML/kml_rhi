use ash::Entry;
use ash::vk;

use crate::{Error, InstanceDesc};

pub struct VulkanInstance {
    entry_loader: Entry,
}

impl VulkanInstance {
    pub unsafe fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        let entry = Entry::load().unwrap();

        let application_info = vk::ApplicationInfo::default()
            .api_version(vk::API_VERSION_1_3);

        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info);

        todo!()
    }
}
