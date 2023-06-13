use std::{net::SocketAddr, str::FromStr};

use axum::{middleware, routing::get, Router};
use groups::{groups_handler, groups_handler_as_array, user_handler};
use iap_verification::iap_verify;

mod groups;
mod iap_verification;
mod jwt;
mod key_provider;
mod token;

mod models;

extern crate dotenv;
use dotenv::dotenv;

use crate::jwt::Parser;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let parser = Parser::new();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/groups", get(groups_handler))
        .route("/groups/slugs", get(groups_handler_as_array))
        .route("/me", get(user_handler))
        .route_layer(middleware::from_fn_with_state(parser.clone(), iap_verify))
        .with_state(parser);

    let port = std::env::var("PORT").unwrap_or(String::from("3000"));
    println!("Listening on port {}", port);

    // convert the port to a socket address
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
