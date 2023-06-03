use crate::models::Claims;
use google_cloud_auth::project::{create_token_source, Config};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json;

pub async fn get_token() -> Result<google_cloud_auth::token::Token, Box<dyn std::error::Error>> {
    let scopes = ["https://www.googleapis.com/auth/cloud-identity.groups.readonly"];
    let sub = "root@ch.tudelft.nl";
    let config = Config {
        // audience is required only for service account jwt-auth
        // https://developers.google.com/identity/protocols/oauth2/service-account#jwt-auth
        audience: None,
        // scopes is required only for service account Oauth2
        // https://developers.google.com/identity/protocols/oauth2/service-account
        scopes: Some(&scopes),
        sub: Some(&sub),
    };
    let ts = create_token_source(config).await?;

    let token = ts.token().await?;

    Ok(token)
}

pub async fn validate_jwt(iap_jwt: String, expected_audience: String) {
    // This URL contains a JSON dictionary that contains a map from the kid claims to the ES256 public keys
    let certs_url = "https://www.gstatic.com/iap/verify/public_key";
    // Fetch the certs from the URL
    let certs = reqwest::get(certs_url)
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    // Get the decoding keys
    let decodingKeys = certs
        .as_array()
        .unwrap()
        .iter()
        .map(|key| DecodingKey::from_ec_pem(&[key]))
        .collect::<Vec<DecodingKey>>();

    // Set the expected issuer and audience
    let issuer = "https://cloud.google.com/iap";

    // Validate the JWT x509
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_audience(&[expected_audience]);
    validation.set_issuer(&[issuer]);

    let token = match decode::<Claims>(&iap_jwt, &certs, &validation) {
        Ok(token) => token,
        Err(e) => {
            println!("Error: {:?}", e);
            Ok(())
        }
    }
    println!("Token: {:?}", token.claims);
    println!("Token: {:?}", token.header);
}

pub async fn token_handler() {
    let token = get_token().await.unwrap();
    println!("token: {:?}", token);
}
