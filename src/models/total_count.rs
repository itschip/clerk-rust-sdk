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
pub struct TotalCount {
    /// String representing the object's type. Objects of the same type share the same value. 
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "total_count")]
    pub total_count: i64,
}

impl TotalCount {
    pub fn new(object: Object, total_count: i64) -> TotalCount {
        TotalCount {
            object,
            total_count,
        }
    }
}

/// String representing the object's type. Objects of the same type share the same value. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "total_count")]
    TotalCount,
}

impl Default for Object {
    fn default() -> Object {
        Self::TotalCount
    }
}

