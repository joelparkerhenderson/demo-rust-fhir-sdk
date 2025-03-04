use fhir_sdk::r5::{
    codes::AdministrativeGender,
    resources::{IdentifiableResource, Patient},
    types::{Identifier, HumanName},
};

#[tokio::main]
async fn main() {
    // Create a Patient resource using a builder.
    let patient = Patient::builder()
        .active(false)
        .identifier(vec![Some(
            Identifier::builder()
                .system("MySystem".to_owned())
                .value("ID".to_owned())
                .build()
                .unwrap()
        )])
        .gender(AdministrativeGender::Male)
        .name(vec![Some(HumanName::builder().family("Test".to_owned()).build().unwrap())])
        .build()
        .unwrap();

    // Check the identifier value.
    assert_eq!(patient.identifier_with_system("MySystem").map(String::as_str), Some("ID"));

    println!("Patient: {:#?}", patient);
}
