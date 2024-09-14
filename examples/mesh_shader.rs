use kml_rhi::{Instance, InstanceDesc, InstanceFlags};

fn main() {
    let instance = unsafe {
        Instance::new(&InstanceDesc {
            flags: InstanceFlags::ENABLE_VALIDATION,
            ..Default::default()
        })
    }
    .unwrap();

    let physical_devices = instance.get_physical_devices();
    for physical_device in physical_devices {
        println!("{}", physical_device.get_name());
    }
}
