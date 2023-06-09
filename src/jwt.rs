use std::sync::Arc;

use crate::key_provider::{GoogleKeyProviderError, GooglePublicKeyProvider};
use jsonwebtoken::{Algorithm, Validation};
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
#[derive(Clone)]
pub struct Parser {
    key_provider: Arc<tokio::sync::Mutex<GooglePublicKeyProvider>>,
}

impl Parser {
    pub const GOOGLE_CERT_URL: &'static str = "https://www.gstatic.com/iap/verify/public_key-jwk";

    pub fn new() -> Self {
        Parser::new_with_custom_url(Parser::GOOGLE_CERT_URL)
    }

    pub fn new_with_custom_url(public_key_url: &str) -> Self {
        Self {
            key_provider: Arc::new(tokio::sync::Mutex::new(GooglePublicKeyProvider::new(
                public_key_url,
            ))),
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
