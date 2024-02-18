// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed.getPostThread` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<crate::types::LimitedU16<1000u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_height: Option<crate::types::LimitedU16<1000u16>>,
    pub uri: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub thread: OutputThreadEnum,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    NotFound(Option<String>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum OutputThreadEnum {
    #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
    AppBskyFeedDefsThreadViewPost(Box<crate::app::bsky::feed::defs::ThreadViewPost>),
    #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
    AppBskyFeedDefsNotFoundPost(Box<crate::app::bsky::feed::defs::NotFoundPost>),
    #[serde(rename = "app.bsky.feed.defs#blockedPost")]
    AppBskyFeedDefsBlockedPost(Box<crate::app::bsky::feed::defs::BlockedPost>),
}
