use axum::Json;
use serde::Serialize;

// Define a struct to represent a group
#[derive(Serialize)]
pub struct Group {
    name: String,
}

// Define a route handler that returns an array of group names
pub async fn groups_handler() -> Json<Vec<Group>> {
    let group_names = vec![
        Group { name: String::from("beheer") },
        Group { name: String::from("lucie") },
        Group { name: String::from("choco") },
    ];

    Json(group_names)
}