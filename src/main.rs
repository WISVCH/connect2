use std::{net::SocketAddr, str::FromStr};

use axum::{routing::get, Router};
use groups::{groups_handler, groups_handler_as_array};
mod groups;
mod token;

mod models;

extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/groups/:member_email", get(groups_handler))
        .route("/groups/:member_email/slugs", get(groups_handler_as_array));

    let port = std::env::var("PORT").unwrap_or(String::from("3000"));
    println!("Listening on port {}", port);

    // convert the port to a socket address
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
