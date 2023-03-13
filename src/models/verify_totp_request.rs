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
pub struct VerifyTotpRequest {
    /// The TOTP or backup code to verify
    #[serde(rename = "code")]
    pub code: String,
}

impl VerifyTotpRequest {
    pub fn new(code: String) -> VerifyTotpRequest {
        VerifyTotpRequest {
            code,
        }
    }
}


