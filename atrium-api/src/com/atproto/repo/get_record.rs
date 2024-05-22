// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.repo.getRecord` namespace.
pub const NSID: &str = "com.atproto.repo.getRecord";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///The CID of the version of the record. If not specified, then return the most recent version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<crate::types::string::Cid>,
    ///The NSID of the record collection.
    pub collection: crate::types::string::Nsid,
    ///The handle or DID of the repo.
    pub repo: crate::types::string::AtIdentifier,
    ///The Record Key.
    pub rkey: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<crate::types::string::Cid>,
    pub uri: String,
    pub value: crate::records::Record,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
