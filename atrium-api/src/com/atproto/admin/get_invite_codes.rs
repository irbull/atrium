// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.getInviteCodes` namespace.
pub const NSID: &str = "com.atproto.admin.getInviteCodes";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU16<500u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub codes: Vec<crate::com::atproto::server::defs::InviteCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
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
