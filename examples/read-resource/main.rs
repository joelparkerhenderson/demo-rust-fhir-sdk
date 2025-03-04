use fhir_sdk::r5::resources::Resource;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_str = r#"{
        "resourceType": "Patient",
        "id": "my-id",
        "birthDate": "2024-01-01"
    }"#;
    let resource: Resource = serde_json::from_str(resource_str)?;
    println!("{:#?}", resource);
    Ok(())
}
