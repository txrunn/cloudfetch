# Cloudfetch

Cloudfetch is a Rust-based system information tool that displays relevant metadata for cloud instances. It supports multiple cloud providers, such as AWS, GCP, and Azure. Cloudfetch is designed to be highly extensible and maintainable with customizable output formats, themes, and user-defined data.

## Features

- Cloud Provider Detection
- Metadata Fetching
- Instance Tags & IAM Role
- User-Defined Data
- Cross-Provider Compatibility
- Formatting & Themes
- Configuration
- Output Formats

## Directory Structure

cloudfetch/
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── formatter.rs
│   ├── connectors/
│   │   ├── mod.rs
│   │   ├── aws.rs
│   │   ├── gcp.rs
│   │   └── azure.rs
│   └── utils/
│       └── mod.rs
├── tests/
│   ├── aws.rs
│   ├── gcp.rs
│   └── azure.rs
├── Cargo.toml
└── README.md

## Code Structure

### Main module (src/main.rs)

- Handles command-line arguments
- Parses configuration file
- Calls the appropriate cloud provider connector
- Formats output using the metadata formatter

### Configuration parser (src/config.rs)

- Reads and validates configuration file or command-line options provided by the user

### Metadata formatter (src/formatter.rs)

- Takes parsed metadata and formats it according to user preferences, themes, and output formats

### Cloud provider connectors (src/connectors/)

- Separate modules for each cloud provider (e.g., AWS, GCP, Azure)
- Handle provider detection, metadata fetching, and metadata parsing

## Getting Started

1. Clone the repository:

```
git clone https://github.com/yourusername/cloudfetch.git
cd cloudfetch
```


2. Build the project:

```cargo build --release```


3. Run Cloudfetch:

```./target/release/cloudfetch```


## Contributing

We welcome contributions to Cloudfetch! Feel free to open issues or submit pull requests for improvements, bug fixes, or new features.

## License

This project is licensed under the [MIT License](LICENSE).

