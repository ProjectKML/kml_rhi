use ash::Entry;

use crate::{Error, InstanceDesc};

pub struct VulkanInstance {
    entry_loader: Entry,
}

impl VulkanInstance {
    pub unsafe fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        let entry = Entry::load().unwrap();
        todo!()
    }
}
