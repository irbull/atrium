// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.label.subscribeLabels` namespace.
pub const NSID: &str = "com.atproto.label.subscribeLabels";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///The last known event to backfill from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    FutureCursor(Option<String>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    pub labels: Vec<crate::com::atproto::label::defs::Label>,
    pub seq: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum Message {
    #[serde(rename = "com.atproto.label.subscribeLabels#labels")]
    Labels(Box<Labels>),
    #[serde(rename = "com.atproto.label.subscribeLabels#info")]
    Info(Box<Info>),
}
