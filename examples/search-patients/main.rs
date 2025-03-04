use fhir_sdk::r5::resources::Patient;
use fhir_sdk::client::{*, r5::*};
use fhir_sdk::{TryStreamExt, header};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up the client using the local test server.
    let settings = RequestSettings::default()
        .header(header::AUTHORIZATION, "Bearer <token>".parse().unwrap());
    let client = Client::builder()
        .base_url("http://localhost:8100/fhir/".parse().unwrap())
        .request_settings(settings)
        .build()?;

    // Create a Patient resource using a builder.
    let mut patient = Patient::builder().active(false).build().unwrap();
    // Push it to the server.
    patient.create(&client).await?;
    // The id and versionId is updated automatically this way.
    assert!(patient.id.is_some());
    
    // Search for all patient with `active` = false, including pagination.
    let patients: Vec<Patient> = client
        .search(SearchParameters::empty().and(TokenSearch::Standard {
            name: "active",
            system: None,
            code: Some("false"),
            not: false,
        }))
        .await?
        .all_matches()
        .try_collect()
        .await?;

    println!("Patients: {:#?}", patients);
    Ok(())
}
