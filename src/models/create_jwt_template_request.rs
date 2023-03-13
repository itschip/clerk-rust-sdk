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
pub struct CreateJwtTemplateRequest {
    /// JWT template name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// JWT template claims in JSON format
    #[serde(rename = "claims", skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    /// JWT token lifetime
    #[serde(rename = "lifetime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<Option<f32>>,
    /// JWT token allowed clock skew
    #[serde(rename = "allowed_clock_skew", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allowed_clock_skew: Option<Option<f32>>,
    /// Whether a custom signing key/algorithm is also provided for this template
    #[serde(rename = "custom_signing_key", skip_serializing_if = "Option::is_none")]
    pub custom_signing_key: Option<bool>,
    /// The custom signing algorithm to use when minting JWTs
    #[serde(rename = "signing_algorithm", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<Option<String>>,
    /// The custom signing private key to use when minting JWTs
    #[serde(rename = "signing_key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<Option<String>>,
}

impl CreateJwtTemplateRequest {
    pub fn new() -> CreateJwtTemplateRequest {
        CreateJwtTemplateRequest {
            name: None,
            claims: None,
            lifetime: None,
            allowed_clock_skew: None,
            custom_signing_key: None,
            signing_algorithm: None,
            signing_key: None,
        }
    }
}


