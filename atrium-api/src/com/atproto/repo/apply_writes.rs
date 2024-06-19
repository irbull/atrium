// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.repo.applyWrites` namespace.
pub const NSID: &str = "com.atproto.repo.applyWrites";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///The handle or DID of the repo (aka, current account).
    pub repo: crate::types::string::AtIdentifier,
    ///If provided, the entire operation will fail if the current repo commit CID does not match this value. Used to prevent conflicting repo mutations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<crate::types::string::Cid>,
    ///Can be set to 'false' to skip Lexicon schema validation of record data, for all operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
    pub writes: Vec<InputWritesItem>,
}
pub type Input = crate::types::Object<InputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    ///Indicates that the 'swapCommit' parameter did not match current commit.
    InvalidSwap(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidSwap(msg) => {
                write!(_f, "InvalidSwap")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
///Operation which creates a new record.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateData {
    pub collection: crate::types::string::Nsid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,
    pub value: crate::records::Record,
}
pub type Create = crate::types::Object<CreateData>;
///Operation which deletes an existing record.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteData {
    pub collection: crate::types::string::Nsid,
    pub rkey: String,
}
pub type Delete = crate::types::Object<DeleteData>;
///Operation which updates an existing record.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateData {
    pub collection: crate::types::string::Nsid,
    pub rkey: String,
    pub value: crate::records::Record,
}
pub type Update = crate::types::Object<UpdateData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum InputWritesItem {
    #[serde(rename = "com.atproto.repo.applyWrites#create")]
    Create(Box<Create>),
    #[serde(rename = "com.atproto.repo.applyWrites#update")]
    Update(Box<Update>),
    #[serde(rename = "com.atproto.repo.applyWrites#delete")]
    Delete(Box<Delete>),
}
