// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.sync.subscribeRepos` namespace.

// com.atproto.sync.subscribeRepos
/// Subscribe to repo updates
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Main {}

// com.atproto.sync.subscribeRepos#commit
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    // pub blobs: Vec<...>
    /// CAR file containing relevant blocks
    // pub blocks: ...,
    // pub commit: ...,
    pub ops: Vec<RepoOp>,
    // pub prev: ...,
    pub rebase: bool,
    pub repo: String,
    pub seq: i32,
    pub time: String,
    pub too_big: bool,
}

// com.atproto.sync.subscribeRepos#handle
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Handle {
    pub did: String,
    pub handle: String,
    pub seq: i32,
    pub time: String,
}

// com.atproto.sync.subscribeRepos#info
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub name: String,
}

// com.atproto.sync.subscribeRepos#migrate
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Migrate {
    pub did: String,
    pub migrate_to: String,
    pub seq: i32,
    pub time: String,
}

// com.atproto.sync.subscribeRepos#repoOp
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RepoOp {
    pub action: String,
    // pub cid: ...,
    pub path: String,
}

// com.atproto.sync.subscribeRepos#tombstone
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tombstone {
    pub did: String,
    pub seq: i32,
    pub time: String,
}
