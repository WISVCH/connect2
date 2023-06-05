use crate::{jwt::Parser, models::TokenClaims};
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn iap_verify<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let iap_header = req
        .headers()
        .get("X-Goog-IAP-JWT-Assertion")
        .and_then(|header| header.to_str().ok());

    let iap_header = if let Some(iap_header) = iap_header {
        iap_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(iap_context) = validate_iap_header(iap_header).await {
        // insert the iap context into a request extension so the handler can access it
        req.extensions_mut().insert(iap_context);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Clone)]
pub struct IapContext {
    pub email: String,
    pub verified: bool,
}

async fn validate_iap_header(header: &str) -> Option<IapContext> {
    // Extract the JWT token from the IAP header
    let jwt_token = header.trim().to_string();

    let parser = Parser::new();
    match parser.parse::<TokenClaims>(&jwt_token).await {
        Ok(claims) => Some(IapContext {
            email: claims.email,
            verified: true,
        }),
        Err(_err) => None,
    }
}
