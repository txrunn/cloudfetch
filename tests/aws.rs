use cloudfetch::connectors::aws::AWS;
use cloudfetch::connectors::Provider;

// Add more tests specific to the AWS connector

#[test]
fn test_aws_fetch_metadata() {
    let aws = AWS {};
    let metadata = aws.fetch_metadata();

    // Add assertions to check if the fetched metadata is correct
    // Example:
    // assert_eq!(metadata.instance_id, "expected_instance_id");
}
