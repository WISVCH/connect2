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
        .route("/", get(|| async { "Hello, World!" }))
        .route("/groups/:member_email", get(groups_handler))
        .route("/groups/:member_email/slugs", get(groups_handler_as_array));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
