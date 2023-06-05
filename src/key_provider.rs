use std::collections::HashMap;
use std::time::Instant;

use headers::Header;
use jsonwebtoken::errors::Error;
use jsonwebtoken::DecodingKey;
use reqwest::header::{HeaderMap, CACHE_CONTROL};
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Clone)]
pub struct GoogleKeys {
    keys: Vec<GoogleKey>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleKey {
    kid: String,
    x: String,
    y: String,
}

#[derive(Error, Debug)]
pub enum GoogleKeyProviderError {
    #[error("key not found")]
    KeyNotFound,
    #[error("network error {0}")]
    FetchError(String),
    #[error("parse error {0}")]
    ParseError(String),
    #[error("create key error {0}")]
    CreateKeyError(Error),
}

#[derive(Debug, Clone)]
pub struct GooglePublicKeyProvider {
    url: String,
    keys: HashMap<String, GoogleKey>,
    expiration_time: Option<Instant>,
}

impl GooglePublicKeyProvider {
    pub fn new(public_key_url: &str) -> Self {
        Self {
            url: public_key_url.to_owned(),
            keys: Default::default(),
            expiration_time: None,
        }
    }

    pub async fn reload(&mut self) -> Result<(), GoogleKeyProviderError> {
        match reqwest::get(&self.url).await {
            Ok(r) => {
                let expiration_time = GooglePublicKeyProvider::parse_expiration_time(r.headers());
                match r.json::<GoogleKeys>().await {
                    Ok(google_keys) => {
                        self.keys.clear();
                        for key in google_keys.keys.into_iter() {
                            self.keys.insert(key.kid.clone(), key);
                        }
                        self.expiration_time = expiration_time;
                        Result::Ok(())
                    }
                    Err(e) => Result::Err(GoogleKeyProviderError::ParseError(format!("{:?}", e))),
                }
            }
            Err(e) => Result::Err(GoogleKeyProviderError::FetchError(format!("{:?}", e))),
        }
    }

    fn parse_expiration_time(header_map: &HeaderMap) -> Option<Instant> {
        match headers::CacheControl::decode(&mut header_map.get_all(CACHE_CONTROL).iter()) {
            Ok(header) => header.max_age().map(|max_age| Instant::now() + max_age),
            Err(_) => None,
        }
    }

    pub fn is_expire(&self) -> bool {
        if let Some(expire) = self.expiration_time {
            Instant::now() > expire
        } else {
            false
        }
    }

    pub async fn get_key(&mut self, kid: &str) -> Result<DecodingKey, GoogleKeyProviderError> {
        if self.expiration_time.is_none() || self.is_expire() {
            self.reload().await?
        }
        match self.keys.get(&kid.to_owned()) {
            None => Result::Err(GoogleKeyProviderError::KeyNotFound),
            Some(key) => DecodingKey::from_ec_components(key.x.as_str(), key.y.as_str())
                .map_err(GoogleKeyProviderError::CreateKeyError),
        }
    }
}
