use std::{net::SocketAddr, str::FromStr};

use axum::{routing::get, Router, middleware};
use groups::{groups_handle, groups_handler_as_array, user_handle};
use iap_verification::iap_verify;

mod groups;
mod token;
mod jwt;
mod key_provider;
mod iap_verification;

mod models;

extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/groups", get(groups_handle))
        .route("/groups/slugs", get(groups_handler_as_array))
        .route("/me", get(user_handle))
        .route_layer(middleware::from_fn(iap_verify));

    let port = std::env::var("PORT").unwrap_or(String::from("3000"));
    println!("Listening on port {}", port);

    // convert the port to a socket address
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
