use kml_rhi::{Instance, InstanceDesc};

fn main() {
    let instance = unsafe { Instance::new(&InstanceDesc::default()) }.unwrap();
}
