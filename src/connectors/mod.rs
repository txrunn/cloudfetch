pub mod aws;
pub mod gcp;
pub mod azure;

use crate::config::Config;

pub trait Provider {
    fn fetch_metadata(&self) -> Metadata;
}

pub struct Metadata {
    // Add common metadata fields here
}

pub fn detect_provider() -> Box<dyn Provider> {
    // Implement logic to detect the cloud provider and return the appropriate connector
}
