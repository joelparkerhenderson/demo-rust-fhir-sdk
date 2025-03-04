use fhir_sdk::r5::resources::Patient;
use fhir_sdk::client::*;
use fhir_sdk::{HttpClient, HeaderValue};

/// Gets called whenever there is an UNAUTHORIZED response.
/// Retries the response with the new Authorization header.
async fn my_auth_callback(client: HttpClient) -> Result<HeaderValue, anyhow::Error> {
    let _response = client.get("my-url").send().await?;
    Ok(HeaderValue::from_static("Bearer <token>"))
}

/// Same as above, but with state.
struct MyLogin {
    valid: std::time::Instant,
}

impl LoginManager for MyLogin {
    type Error = anyhow::Error;
    
    async fn authenticate(&mut self, client: HttpClient) -> Result<HeaderValue, Self::Error> {
        if self.valid.elapsed().as_secs() > 360 {
            let _response = client.get("my-url").send().await?;
            self.valid = std::time::Instant::now();
        }
        Ok(HeaderValue::from_static("Bearer <token>"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up the client using the local test server.
    let client = Client::builder()
        .base_url("http://localhost:8100/fhir/".parse().unwrap())
        // Register async fn as callback.
        .auth_callback(my_auth_callback)
        // Register struct with state. Overwrites previous callback.
        .auth_callback(MyLogin { valid: std::time::Instant::now() })
        // Register async closure. Overwrites previous callback.
        .auth_callback(|_client: HttpClient| async move { anyhow::Ok(HeaderValue::from_static("hi")) })
        .build()?;

    // Create a Patient resource using a builder.
    let mut patient = Patient::builder().active(false).build().unwrap();
    // Push it to the server. On unauthorized failures, the client will call our
    // auth_callback method to refresh the authorization.
    patient.create(&client).await?;

    Ok(())
}