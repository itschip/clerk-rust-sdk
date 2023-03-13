/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.dev/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActorToken {
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "actor")]
    pub actor: serde_json::Value,
    #[serde(rename = "token", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub token: Option<Option<String>>,
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    /// Unix timestamp of creation. 
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Unix timestamp of last update. 
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl ActorToken {
    pub fn new(object: Object, id: String, status: Status, user_id: String, actor: serde_json::Value, created_at: i64, updated_at: i64) -> ActorToken {
        ActorToken {
            object,
            id,
            status,
            user_id,
            actor,
            token: None,
            url: None,
            created_at,
            updated_at,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "actor_token")]
    ActorToken,
}

impl Default for Object {
    fn default() -> Object {
        Self::ActorToken
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "revoked")]
    Revoked,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

