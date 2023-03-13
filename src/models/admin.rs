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
pub struct Admin {
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "strategy")]
    pub strategy: Strategy,
    #[serde(rename = "attempts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Option<i32>>,
    #[serde(rename = "expire_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<Option<i32>>,
}

impl Admin {
    pub fn new(status: Status, strategy: Strategy) -> Admin {
        Admin {
            status,
            strategy,
            attempts: None,
            expire_at: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "verified")]
    Verified,
}

impl Default for Status {
    fn default() -> Status {
        Self::Verified
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "admin")]
    Admin,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::Admin
    }
}

