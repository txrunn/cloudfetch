use crate::config::Config;
use crate::connectors::{Provider, Metadata};

pub struct AWS {}

impl Provider for AWS {
    fn fetch_metadata(&self) -> Metadata {
        // Fetch and parse metadata from the AWS metadata API

        Metadata {
            // Initialize common metadata fields
        }
    }
}
