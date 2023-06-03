use google_cloud_auth::project::{create_token_source, Config};

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
