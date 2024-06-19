// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.identity.getRecommendedDidCredentials` namespace.
pub const NSID: &str = "com.atproto.identity.getRecommendedDidCredentials";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,
    ///Recommended rotation keys for PLC dids. Should be undefined (or ignored) for did:webs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<crate::records::Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_methods: Option<crate::records::Record>,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
