use std::time::Instant;

use dotenv::dotenv;

use crate::{
    iap_verification::IapContext,
    models::{
        Group, SearchTransitiveGroupsResponse, VerifiedResponseGroups, VerifiedResponseGroupsSlug,
    },
    token::get_token, AppState, cache_lifespan_in_seconds,
};
use axum::{Extension, Json, extract::State};

pub async fn groups_handle(
    State(app_state) : State<AppState>,
    Extension(iap_context): Extension<IapContext>,
) -> Json<VerifiedResponseGroups> {
    let groups = get_groups_with_cache(&app_state, &iap_context).await;

    let verified_response = VerifiedResponseGroups {
        email: iap_context.email,
        groups,
    };
    Json(verified_response)
}
pub async fn groups_handler_as_array(
    State(app_state): State<AppState>,
    Extension(iap_context): Extension<IapContext>,
) -> Json<VerifiedResponseGroupsSlug> {
    let groups = get_groups_with_cache(&app_state, &iap_context).await;

    let mut slugs = vec![];
    for group in groups {
        slugs.push(group.slug);
    }
    let verified_response = VerifiedResponseGroupsSlug {
        email: iap_context.email,
        groups: slugs,
    };
    Json(verified_response)
}

pub async fn user_handle(Extension(iap_context): Extension<IapContext>) -> Json<String> {
    Json(iap_context.email)
}

async fn get_groups_with_cache(app_state: &AppState, iap_context: &IapContext) -> Vec<Group> {
    match app_state.user_group_cache.read().unwrap().get(&iap_context.email) {
        Some(cached_groups) => {
            if cached_groups.0.elapsed().as_secs() < cache_lifespan_in_seconds {
                cached_groups.1.clone()
            } else {
                let groupResult = get_groups(&iap_context.email).await.unwrap();
                app_state.user_group_cache.write().unwrap().insert(iap_context.email.clone(), (Instant::now(), groupResult.clone()));
                groupResult
            }
        }  .clone(),
        None =>  {
            let groupResult = get_groups(&iap_context.email).await.unwrap();
            app_state.user_group_cache.write().unwrap().insert(iap_context.email.clone(), (Instant::now(), groupResult.clone()));
            groupResult
        },
    }
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
