use crate::config::Config;
use crate::connectors::{Provider, Metadata};

pub struct GCP {}

impl Provider for GCP {
    fn fetch_metadata(&self) -> Metadata {
        // Fetch and parse metadata from the GCP metadata API

        Metadata {
            // Initialize common metadata fields
        }
    }
}
