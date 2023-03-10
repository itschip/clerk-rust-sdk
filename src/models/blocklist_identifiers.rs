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
pub struct BlocklistIdentifiers {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::BlocklistIdentifier>,
    /// Total number of blocklist identifiers 
    #[serde(rename = "total_count")]
    pub total_count: i64,
}

impl BlocklistIdentifiers {
    pub fn new(data: Vec<crate::models::BlocklistIdentifier>, total_count: i64) -> BlocklistIdentifiers {
        BlocklistIdentifiers {
            data,
            total_count,
        }
    }
}


