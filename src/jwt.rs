use crate::{
    models::TokenClaims,
    key_provider::{GoogleKeyProviderError, GooglePublicKeyProvider},
};
use axum::{
    http::header::{HeaderMap},
};
use jsonwebtoken::{Validation, Algorithm};
use serde::de::DeserializeOwned;
use thiserror::Error;

///
/// Parser errors
///
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Wrong header.")]
    WrongHeader,
    #[error("Unknown kid.")]
    UnknownKid,
    #[error("Download public key error - {0}.")]
    KeyProvider(GoogleKeyProviderError),
    #[error("Wrong token format - {0}.")]
    WrongToken(jsonwebtoken::errors::Error),
}

///
/// Parse & Validate Google JWT token.
/// Use public key from http(s) server.
///
pub struct Parser {
    key_provider: tokio::sync::Mutex<GooglePublicKeyProvider>,
}

impl Parser {
    pub const GOOGLE_CERT_URL: &'static str = "https://www.gstatic.com/iap/verify/public_key-jwk";

    pub fn new() -> Self {
        Parser::new_with_custom_url( Parser::GOOGLE_CERT_URL)
    }

    pub fn new_with_custom_url(public_key_url: &str) -> Self {
        Self {
            key_provider: tokio::sync::Mutex::new(GooglePublicKeyProvider::new(public_key_url)),
        }
    }

    ///
    /// Parse and validate token.
    /// Download and cache public keys from http(s) server.
    /// Use expire time header for reload keys.
    ///
    pub async fn parse<T: DeserializeOwned>(&self, token: &str) -> Result<T, ParserError> {
        let mut provider = self.key_provider.lock().await;
        match jsonwebtoken::decode_header(token) {
            Ok(header) => match header.kid {
                None => Result::Err(ParserError::UnknownKid),
                Some(kid) => match provider.get_key(kid.as_str()).await {
                    Ok(key) => {
                        let mut validation = Validation::new(Algorithm::ES256);
                        validation.set_issuer(&["https://cloud.google.com/iap".to_string()]);
                        validation.validate_exp = true;
                        validation.validate_nbf = false;
                        let result = jsonwebtoken::decode::<T>(token, &key, &validation);
                        match result {
                            Result::Ok(token_data) => Result::Ok(token_data.claims),
                            Result::Err(error) => Result::Err(ParserError::WrongToken(error)),
                        }
                    }
                    Err(e) => {
                        let error = ParserError::KeyProvider(e);
                        Result::Err(error)
                    }
                },
            },
            Err(_) => Result::Err(ParserError::WrongHeader),
        }
    }
}

pub async fn validate_jwt(headers: HeaderMap) -> String {

    // Extract the IAP proxy header from the request headers
    let iap_header = match headers.get("X-Goog-IAP-JWT-Assertion") {
        Some(header_value) =>  match header_value.to_str() {
            Ok(token) => token.to_owned(),
            Err(_) => String::from(""),
        },
        None => String::from(""),
    };


    // Check if the IAP header exists and is not empty
    if iap_header.is_empty() {
        return "Invalid or missing IAP header!".to_string();
    }

    // Extract the JWT token from the IAP header
    let jwt_token = iap_header.trim_start_matches("Bearer ").to_string();

    let parser = Parser::new();
    let claims = parser.parse::<TokenClaims>(&jwt_token).await.unwrap();
    return claims.email;
}