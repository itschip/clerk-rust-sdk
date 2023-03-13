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
pub struct BlocklistIdentifier {
    /// String representing the object's type. Objects of the same type share the same value. 
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An email address, email domain, phone number or web3 wallet. 
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "identifier_type", skip_serializing_if = "Option::is_none")]
    pub identifier_type: Option<IdentifierType>,
    #[serde(rename = "instance_id", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Unix timestamp of creation 
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Unix timestamp of last update. 
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

impl BlocklistIdentifier {
    pub fn new() -> BlocklistIdentifier {
        BlocklistIdentifier {
            object: None,
            id: None,
            identifier: None,
            identifier_type: None,
            instance_id: None,
            created_at: None,
            updated_at: None,
        }
    }
}

/// String representing the object's type. Objects of the same type share the same value. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "blocklist_identifier")]
    BlocklistIdentifier,
}

impl Default for Object {
    fn default() -> Object {
        Self::BlocklistIdentifier
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentifierType {
    #[serde(rename = "email_address")]
    EmailAddress,
    #[serde(rename = "phone_number")]
    PhoneNumber,
    #[serde(rename = "web3_wallet")]
    Web3Wallet,
}

impl Default for IdentifierType {
    fn default() -> IdentifierType {
        Self::EmailAddress
    }
}

