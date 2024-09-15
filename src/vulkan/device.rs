use std::sync::Arc;

use ash::{ext::mesh_shader, vk};
use vk_mem_alloc::{Allocator, AllocatorCreateFlags, AllocatorCreateInfo};

use crate::{
    vulkan::{VulkanError, VulkanInstance},
    DeviceDesc, PhysicalDevice,
};

struct Inner {
    device: ash::Device,

    ext_mesh_shader_device: mesh_shader::Device,

    allocator: Allocator,
}

fn find_direct_queue_family_index(properties: &[vk::QueueFamilyProperties]) -> Option<u32> {
    let mut queue_count: u32 = 0;
    let mut family_index: u32 = 0;

    let direct_flags: vk::QueueFlags =
        vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER;

    for (i, properties) in properties.iter().enumerate() {
        let i = i as u32;

        if (properties.queue_flags & direct_flags) == direct_flags
            && properties.queue_count > queue_count
        {
            queue_count = properties.queue_count;
            family_index = i;
        }
    }

    if queue_count > 0 {
        Some(family_index)
    } else {
        None
    }
}

fn find_queue_family_index(
    properties: &[vk::QueueFamilyProperties],
    desired_flags: vk::QueueFlags,
    undesired_flags: vk::QueueFlags,
) -> Option<u32> {
    let mut queue_count: u32 = 0;
    let mut family_index: u32 = 0;

    for (i, properties) in properties.iter().enumerate() {
        let i = i as u32;

        if (properties.queue_flags & desired_flags) == desired_flags
            && (properties.queue_flags & undesired_flags) == vk::QueueFlags::empty()
            && properties.queue_count > queue_count
        {
            queue_count = properties.queue_count;
            family_index = i;
        }
    }

    if queue_count > 0 {
        Some(family_index)
    } else {
        None
    }
}

fn find_queue_family_indices(properties: &[vk::QueueFamilyProperties]) -> Option<(u32, u32, u32)> {
    let direct_index = find_direct_queue_family_index(properties)?;
    let compute_index = find_queue_family_index(
        properties,
        vk::QueueFlags::COMPUTE,
        vk::QueueFlags::GRAPHICS | vk::QueueFlags::TRANSFER,
    )
    .or_else(|| {
        find_queue_family_index(
            properties,
            vk::QueueFlags::COMPUTE,
            vk::QueueFlags::GRAPHICS,
        )
    })
    .or_else(|| {
        find_queue_family_index(
            properties,
            vk::QueueFlags::COMPUTE,
            vk::QueueFlags::TRANSFER,
        )
    })
    .unwrap_or(direct_index);

    let transfer_index = find_queue_family_index(
        properties,
        vk::QueueFlags::TRANSFER,
        vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE,
    )
    .or_else(|| {
        find_queue_family_index(
            properties,
            vk::QueueFlags::TRANSFER,
            vk::QueueFlags::GRAPHICS,
        )
    })
    .or_else(|| {
        find_queue_family_index(
            properties,
            vk::QueueFlags::TRANSFER,
            vk::QueueFlags::COMPUTE,
        )
    })
    .unwrap_or(direct_index);

    Some((direct_index, compute_index, transfer_index))
}

#[derive(Clone)]
pub struct VulkanDevice(Arc<Inner>);

impl VulkanDevice {
    pub fn new(instance: &VulkanInstance, desc: &DeviceDesc) -> Result<Self, VulkanError> {
        let PhysicalDevice::Vulkan(physical_device) = &desc.physical_device else {
            return Err(VulkanError::Custom("Invalid physical device".to_owned()));
        };

        let queue_family_properties = unsafe {
            instance
                .instance()
                .get_physical_device_queue_family_properties(*physical_device.physical_device())
        };

        let (direct_queue_family_index, compute_queue_family_index, transfer_queue_family_index) =
            find_queue_family_indices(&queue_family_properties).ok_or(VulkanError::Custom(
                "Failed to find queue family indices".to_owned(),
            ))?;

        let queue_priorities = [1.0];

        let mut device_queue_create_infos = vec![vk::DeviceQueueCreateInfo::default()
            .queue_family_index(direct_queue_family_index)
            .queue_priorities(&queue_priorities)];

        if compute_queue_family_index != direct_queue_family_index {
            device_queue_create_infos.push(
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(compute_queue_family_index)
                    .queue_priorities(&queue_priorities),
            );
        }

        if transfer_queue_family_index != direct_queue_family_index {
            device_queue_create_infos.push(
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(transfer_queue_family_index)
                    .queue_priorities(&queue_priorities),
            );
        }

        let device_create_info =
            vk::DeviceCreateInfo::default().queue_create_infos(&device_queue_create_infos);

        let device = unsafe {
            instance.instance().create_device(
                *physical_device.physical_device(),
                &device_create_info,
                None,
            )
        }?;

        let ext_mesh_shader_device = mesh_shader::Device::new(instance.instance(), &device);

        let allocator = unsafe {
            vk_mem_alloc::create_allocator(
                instance.instance(),
                *physical_device.physical_device(),
                &device,
                Some(&AllocatorCreateInfo {
                    flags: AllocatorCreateFlags::BUFFER_DEVICE_ADDRESS,
                    ..Default::default()
                }),
            )
        }?;

        Ok(Self(Arc::new(Inner {
            device,

            ext_mesh_shader_device,
            allocator,
        })))
    }

    #[inline]
    pub fn device(&self) -> &ash::Device {
        &self.0.device
    }

    #[inline]
    pub fn ext_mesh_shader_device(&self) -> &mesh_shader::Device {
        &self.0.ext_mesh_shader_device
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
