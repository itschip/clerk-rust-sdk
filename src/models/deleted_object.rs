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
pub struct DeletedObject {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl DeletedObject {
    pub fn new(object: String, deleted: bool) -> DeletedObject {
        DeletedObject {
            object,
            id: None,
            slug: None,
            deleted,
        }
    }
}

