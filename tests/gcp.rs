use cloudfetch::connectors::gcp::GCP;
use cloudfetch::connectors::Provider;

// Add more tests specific to the GCP connector

#[test]
fn test_gcp_fetch_metadata() {
    let gcp = GCP {};
    let metadata = gcp.fetch_metadata();

    // Add assertions to check if the fetched metadata is correct
    // Example:
    // assert_eq!(metadata.instance_id, "expected_instance_id");
}
