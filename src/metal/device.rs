use std::sync::Arc;

struct Inner {}

#[derive(Clone)]
pub struct MetalDevice(Arc<Inner>);
