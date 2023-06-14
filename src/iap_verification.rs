use crate::{
    jwt::{Parser, ParserError},
    models::TokenClaims,
};
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn iap_verify<B>(
    State(parser): State<Parser>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let iap_header = req
        .headers()
        .get("X-Goog-IAP-JWT-Assertion")
        .and_then(|header| header.to_str().ok());

    let iap_header = if let Some(iap_header) = iap_header {
        iap_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    match validate_iap_header(parser, iap_header).await {
        Ok(claims) => {
            // insert the iap context into a request extension so the handler can access it
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(err) => {
            eprintln!("Error validating IAP header {}: {:?}", iap_header, err);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

#[derive(Clone)]
pub struct IapContext {
    pub email: String,
    pub verified: bool,
}

async fn validate_iap_header(parser: Parser, header: &str) -> Result<IapContext, ParserError> {
    // Extract the JWT token from the IAP header
    let jwt_token = header.trim().to_string();

    match parser.parse::<TokenClaims>(&jwt_token).await {
        Ok(claims) => Ok(IapContext {
            email: claims.email,
            verified: true,
        }),
        Err(err) => Err(err),
    }
}
