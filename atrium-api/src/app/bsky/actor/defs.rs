// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.actor.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdultContentPrefData {
    pub enabled: bool,
}
pub type AdultContentPref = crate::types::Object<AdultContentPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContentLabelPrefData {
    pub label: String,
    ///Which labeler does this preference apply to? If undefined, applies globally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeler_did: Option<crate::types::string::Did>,
    pub visibility: String,
}
pub type ContentLabelPref = crate::types::Object<ContentLabelPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedViewPrefData {
    ///The URI of the feed, or an identifier which describes the feed.
    pub feed: String,
    ///Hide quote posts in the feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_quote_posts: Option<bool>,
    ///Hide replies in the feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_replies: Option<bool>,
    ///Hide replies in the feed if they do not have this number of likes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_replies_by_like_count: Option<i64>,
    ///Hide replies in the feed if they are not by followed users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_replies_by_unfollowed: Option<bool>,
    ///Hide reposts in the feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_reposts: Option<bool>,
}
pub type FeedViewPref = crate::types::Object<FeedViewPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HiddenPostsPrefData {
    ///A list of URIs of posts the account owner has hidden.
    pub items: Vec<String>,
}
pub type HiddenPostsPref = crate::types::Object<HiddenPostsPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InterestsPrefData {
    ///A list of tags which describe the account owner's interests gathered during onboarding.
    pub tags: Vec<String>,
}
pub type InterestsPref = crate::types::Object<InterestsPrefData>;
///The subject's followers whom you also follow
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KnownFollowersData {
    pub count: i64,
    pub followers: Vec<ProfileViewBasic>,
}
pub type KnownFollowers = crate::types::Object<KnownFollowersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelerPrefItemData {
    pub did: crate::types::string::Did,
}
pub type LabelerPrefItem = crate::types::Object<LabelerPrefItemData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelersPrefData {
    pub labelers: Vec<LabelerPrefItem>,
}
pub type LabelersPref = crate::types::Object<LabelersPrefData>;
///A word that the account owner has muted.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutedWordData {
    ///The intended targets of the muted word.
    pub targets: Vec<crate::app::bsky::actor::defs::MutedWordTarget>,
    ///The muted word itself.
    pub value: String,
}
pub type MutedWord = crate::types::Object<MutedWordData>;
pub type MutedWordTarget = String;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutedWordsPrefData {
    ///A list of words the account owner has muted.
    pub items: Vec<crate::app::bsky::actor::defs::MutedWord>,
}
pub type MutedWordsPref = crate::types::Object<MutedWordsPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersonalDetailsPrefData {
    ///The birth date of account owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<crate::types::string::Datetime>,
}
pub type PersonalDetailsPref = crate::types::Object<PersonalDetailsPrefData>;
pub type Preferences = Vec<crate::types::Union<PreferencesItem>>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAssociatedData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<ProfileAssociatedChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedgens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<i64>,
}
pub type ProfileAssociated = crate::types::Object<ProfileAssociatedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAssociatedChatData {
    pub allow_incoming: String,
}
pub type ProfileAssociatedChat = crate::types::Object<ProfileAssociatedChatData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileViewData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub did: crate::types::string::Did,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub handle: crate::types::string::Handle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<crate::types::string::Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}
pub type ProfileView = crate::types::Object<ProfileViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileViewBasicData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub did: crate::types::string::Did,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub handle: crate::types::string::Handle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}
pub type ProfileViewBasic = crate::types::Object<ProfileViewBasicData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileViewDetailedData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub did: crate::types::string::Did,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follows_count: Option<i64>,
    pub handle: crate::types::string::Handle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<crate::types::string::Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}
pub type ProfileViewDetailed = crate::types::Object<ProfileViewDetailedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SavedFeedData {
    pub id: String,
    pub pinned: bool,
    pub r#type: String,
    pub value: String,
}
pub type SavedFeed = crate::types::Object<SavedFeedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SavedFeedsPrefData {
    pub pinned: Vec<String>,
    pub saved: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline_index: Option<i64>,
}
pub type SavedFeedsPref = crate::types::Object<SavedFeedsPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SavedFeedsPrefV2Data {
    pub items: Vec<crate::app::bsky::actor::defs::SavedFeed>,
}
pub type SavedFeedsPrefV2 = crate::types::Object<SavedFeedsPrefV2Data>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ThreadViewPrefData {
    ///Show followed users at the top of all replies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritize_followed_users: Option<bool>,
    ///Sorting mode for threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}
pub type ThreadViewPref = crate::types::Object<ThreadViewPrefData>;
///Metadata about the requesting account's relationship with the subject account. Only has meaningful content for authed requests.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewerStateData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_by: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking_by_list: Option<crate::app::bsky::graph::defs::ListViewBasic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_followers: Option<KnownFollowers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted_by_list: Option<crate::app::bsky::graph::defs::ListViewBasic>,
}
pub type ViewerState = crate::types::Object<ViewerStateData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum PreferencesItem {
    #[serde(rename = "app.bsky.actor.defs#adultContentPref")]
    AdultContentPref(Box<AdultContentPref>),
    #[serde(rename = "app.bsky.actor.defs#contentLabelPref")]
    ContentLabelPref(Box<ContentLabelPref>),
    #[serde(rename = "app.bsky.actor.defs#savedFeedsPref")]
    SavedFeedsPref(Box<SavedFeedsPref>),
    #[serde(rename = "app.bsky.actor.defs#savedFeedsPrefV2")]
    SavedFeedsPrefV2(Box<SavedFeedsPrefV2>),
    #[serde(rename = "app.bsky.actor.defs#personalDetailsPref")]
    PersonalDetailsPref(Box<PersonalDetailsPref>),
    #[serde(rename = "app.bsky.actor.defs#feedViewPref")]
    FeedViewPref(Box<FeedViewPref>),
    #[serde(rename = "app.bsky.actor.defs#threadViewPref")]
    ThreadViewPref(Box<ThreadViewPref>),
    #[serde(rename = "app.bsky.actor.defs#interestsPref")]
    InterestsPref(Box<InterestsPref>),
    #[serde(rename = "app.bsky.actor.defs#mutedWordsPref")]
    MutedWordsPref(Box<MutedWordsPref>),
    #[serde(rename = "app.bsky.actor.defs#hiddenPostsPref")]
    HiddenPostsPref(Box<HiddenPostsPref>),
}
