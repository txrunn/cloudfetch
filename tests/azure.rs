use cloudfetch::connectors::azure::Azure;
use cloudfetch::connectors::Provider;

// Add more tests specific to the Azure connector

#[test]
fn test_azure_fetch_metadata() {
    let azure = Azure {};
    let metadata = azure.fetch_metadata();

    // Add assertions to check if the fetched metadata is correct
    // Example:
    // assert_eq!(metadata.instance_id, "expected_instance_id");
}
