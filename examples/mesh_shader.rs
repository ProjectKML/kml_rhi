use kml_rhi::{BackendType, Instance, InstanceDesc, InstanceFlags};

fn main() {
    let instance = unsafe { Instance::new(&InstanceDesc::default()) }.unwrap();

    let physical_devices = instance.get_physical_devices();
    for physical_device in physical_devices {
        println!("{}", physical_device.get_name());
    }
}
