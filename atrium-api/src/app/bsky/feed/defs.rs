// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.defs` namespace.

// app.bsky.feed.defs#feedViewPost
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FeedViewPost {
    pub post: crate::app::bsky::feed::defs::PostView,
    // pub reason: ...,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
}

// app.bsky.feed.defs#notFoundPost
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotFoundPost {
    pub not_found: bool,
    pub uri: String,
}

// app.bsky.feed.defs#postView
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostView {
    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
    pub cid: String,
    // pub embed: ...,
    pub indexed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    pub record: crate::records::Record,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repost_count: Option<i32>,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,
}

// app.bsky.feed.defs#reasonRepost
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReasonRepost {
    pub by: crate::app::bsky::actor::defs::ProfileViewBasic,
    pub indexed_at: String,
}

// app.bsky.feed.defs#replyRef
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReplyRef {
    pub parent: crate::app::bsky::feed::defs::PostView,
    pub root: crate::app::bsky::feed::defs::PostView,
}

// app.bsky.feed.defs#threadViewPost
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ThreadViewPost {
    // pub parent: ...,
    pub post: PostView,
    // pub replies: Vec<...>
}

// app.bsky.feed.defs#viewerState
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViewerState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repost: Option<String>,
}
