use std::env;
use config::Config;
use formatter::format_metadata;
use connectors::{Provider, detect_provider};

mod config;
mod formatter;
mod connectors;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Load configuration from file or command-line options
    let config = Config::from_args(&args);

    // Detect the cloud provider based on metadata endpoints or other provider-specific methods
    let provider = detect_provider();

    // Fetch and parse metadata from the cloud provider
    let metadata = provider.fetch_metadata();

    // Format the metadata based on user preferences, themes, and output formats
    let formatted_output = format_metadata(metadata, &config);

    // Display the formatted metadata
    println!("{}", formatted_output);
}

