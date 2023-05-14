use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRelation {
    pub group_key: EntityKey,
    group: String,
    pub display_name: String,
    roles: Vec<TransitiveMembershipRole>,
    relation_type: String,
    labels: HashMap<String, String>,
}

// EntityKey
#[derive(Deserialize)]
pub struct EntityKey {
    pub id: String,
    namespace: Option<String>,
}

// Currently supported TransitiveMembershipRoles: "MEMBER", "OWNER", and "MANAGER".

#[derive(Deserialize)]
struct TransitiveMembershipRole {
    role: String
}

/* RelationType (enum)
* RELATION_TYPE_UNSPECIFIED	The relation type is undefined or undetermined.
* DIRECT	The two entities have only a direct membership with each other.
* INDIRECT	The two entities have only an indirect membership with each other.
* DIRECT_AND_INDIRECT	The two entities have both a direct and an indirect membership with each other.
*/

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTransitiveGroupsResponse {
    pub memberships: Vec<GroupRelation>,
    next_page_token: Option<String>
}