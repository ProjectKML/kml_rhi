mod instance;
mod physical_device;

pub use instance::*;
pub use physical_device::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetalError {
    #[error("{0}")]
    Custom(String),
}
