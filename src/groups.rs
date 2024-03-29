use dotenv::dotenv;

use crate::{
    iap_verification::IapContext,
    models::{
        Group, SearchTransitiveGroupsResponse, VerifiedResponseGroups, VerifiedResponseGroupsSlug,
    },
    token::get_token,
};
use axum::{Extension, Json};

pub async fn groups_handler(
    Extension(iap_context): Extension<IapContext>,
) -> Json<VerifiedResponseGroups> {
    let groups = get_groups(&iap_context.email).await.unwrap();
    let verified_response = VerifiedResponseGroups {
        email: iap_context.email,
        groups,
    };
    Json(verified_response)
}
pub async fn groups_handler_as_array(
    Extension(iap_context): Extension<IapContext>,
) -> Json<VerifiedResponseGroupsSlug> {
    let groups = get_groups(&iap_context.email).await.unwrap();
    let mut slugs = vec![];
    for group in groups {
        slugs.push(group.slug);
    }
    let verified_response = VerifiedResponseGroupsSlug {
        email: iap_context.email,
        groups: slugs,
    };

    println!(
        "Authenticated user: {} with groups: {:?}",
        verified_response.email, verified_response.groups
    );

    Json(verified_response)
}

pub async fn user_handler(Extension(iap_context): Extension<IapContext>) -> Json<String> {
    Json(iap_context.email)
}

/* Retrieve the Groups from google workspace.
* We use the `searchTransitiveGroups` method to retrieve all groups (https://cloud.google.com/identity/docs/reference/rest/v1/groups.memberships/searchTransitiveGroups)
*/
pub async fn get_groups(member_email: &str) -> Result<Vec<Group>, Box<dyn std::error::Error>> {
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
        ("query", format!("member_key_id == '{}' && 'cloudidentity.googleapis.com/groups.discussion_forum' in labels && parent == 'customers/{}'",
        member_email, std::env::var("CUSTOMER_ID").unwrap()))
        ])
        .bearer_auth(access_token)
        .send()
        .await;

    match response {
        Ok(res) => {
            let body = res.text().await.unwrap();

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
