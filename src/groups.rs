use dotenv::dotenv;

use crate::{
    models::{Group, SearchTransitiveGroupsResponse},
    token::get_token,
};
use axum::{extract::Path, Json};

pub async fn groups_handler(Path(member_email): Path<String>) -> Json<Vec<Group>> {
    Json(get_groups(member_email).await.unwrap())
}
pub async fn groups_handler_as_array(Path(member_email): Path<String>) -> Json<Vec<String>> {
    let groups = get_groups(member_email).await.unwrap();
    let mut slugs = vec![];
    for group in groups {
        slugs.push(group.slug);
    }
    Json(slugs)
}

/* Retrieve the Groups from google workspace.
* We use the `searchTransitiveGroups` method to retrieve all groups (https://cloud.google.com/identity/docs/reference/rest/v1/groups.memberships/searchTransitiveGroups)
*/
pub async fn get_groups(member_email: String) -> Result<Vec<Group>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let access_token = get_token().await.unwrap().access_token;

    let url = format!(
        "https://cloudidentity.googleapis.com/v1/{}/memberships:searchTransitiveGroups",
        "groups/-"
    );

    let client = reqwest::Client::new();

    let response = client
    .get(&url)
    .query(&[
        ("query", format!("member_key_id == '{}' && 'cloudidentity.googleapis.com/groups.discussion_forum' in labels", member_email))
        ])
        .bearer_auth(access_token)
        .send()
        .await;

    println!("Response: {:?}", response);

    match response {
        Ok(res) => {
            let body = res.text().await.unwrap();
            println!("Response Body: {}", body);

            match serde_json::from_str::<SearchTransitiveGroupsResponse>(&body) {
                Ok(data) => Ok(map_groups_response_to_groups(data)),
                Err(e) => {
                    println!("Error: {:?}", e);
                    Ok(vec![])
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Ok(vec![])
        }
    }
}

fn get_slug_from_email(email: String) -> String {
    /* Replace "@ch.tudelft.nl" from the group names
     * 1. Replace "-commissie@ch.tudelft.nl" with ""
     * 2. Replace "-group@ch.tudelft.nl" with ""
     * 3. Replace "@ch.tudelft.nl" with ""
     */
    email
        .replace("-commissie@ch.tudelft.nl", "")
        .replace("-group@ch.tudelft.nl", "")
        .replace("@ch.tudelft.nl", "")
}
fn map_groups_response_to_groups(response: SearchTransitiveGroupsResponse) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];
    for group in response.memberships {
        groups.push(Group {
            name: group.display_name,
            email: group.group_key.id.clone(),
            slug: get_slug_from_email(group.group_key.id),
        });
    }
    groups
}
