// use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Group {
    pub email: String,
    pub slug: String,
    pub name: String,
}

#[derive(Clone, Serialize)]
pub struct VerifiedResponseGroups {
    pub email: String,
    pub groups: Vec<Group>,
}

#[derive(Clone, Serialize)]
pub struct VerifiedResponseGroupsSlug {
    pub email: String,
    pub groups: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRelation {
    pub group_key: EntityKey,
    // group: String,
    pub display_name: String,
    // roles: Vec<TransitiveMembershipRole>,
    // relation_type: String,
    // labels: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct EntityKey {
    pub id: String,
    // namespace: Option<String>,
}

// #[derive(Deserialize)]
// struct TransitiveMembershipRole {
//     role: String, // "MEMBER", "OWNER", and "MANAGER".
// }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTransitiveGroupsResponse {
    pub memberships: Vec<GroupRelation>,
    // next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    iss: String,
    aud: String,
    // hd: String,
    // google: Google,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
}
