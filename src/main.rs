use std::{
    collections::HashMap,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, RwLock}, time::Instant,
};

use axum::{middleware, routing::get, Router};
use groups::{groups_handle, groups_handler_as_array, user_handle};
use iap_verification::iap_verify;

mod groups;
mod iap_verification;
mod jwt;
mod key_provider;
mod token;

mod models;

extern crate dotenv;
use dotenv::dotenv;
use models::Group;

use crate::jwt::Parser;

pub const cache_lifespan_in_seconds: u64 = 60;

#[derive(Clone)]
pub struct AppState {
    pub parser: Parser,
    pub user_group_cache: Arc<RwLock<HashMap<String, (Instant, Vec<Group>)>>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = AppState {
        parser: Parser::new(),
        user_group_cache: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/groups", get(groups_handle))
        .route("/groups/slugs", get(groups_handler_as_array))
        .route("/me", get(user_handle))
        .route_layer(middleware::from_fn_with_state(state.clone(), iap_verify))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or(String::from("3000"));
    println!("Listening on port {}", port);

    // convert the port to a socket address
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
