use kml_rhi::{BackendType, DeviceDesc, Instance, InstanceDesc, InstanceFlags};

fn main() {
    let instance = unsafe {
        Instance::new(&InstanceDesc {
            backend_type: BackendType::Vulkan,
            ..Default::default()
        })
    }
    .unwrap();

    let physical_devices = instance.get_physical_devices();
    let physical_device = physical_devices[0].clone();

    let device = instance
        .create_device(&DeviceDesc { physical_device })
        .unwrap();
}
