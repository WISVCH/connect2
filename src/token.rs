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
    };
    let mut ts = create_token_source(config).await?;
    ts.set_subject(sub).await;

    println!("ts: {:?}", ts);
    // ts.sub = Some(sub.to_string());
    let token = ts.token().await?;

    // println!("token: {:?}", token);

    Ok(token)
}

pub async fn token_handler() {
    let token = get_token().await.unwrap();
    println!("token: {:?}", token);
}
