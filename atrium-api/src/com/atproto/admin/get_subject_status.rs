// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.getSubjectStatus` namespace.
pub const NSID: &str = "com.atproto.admin.getSubjectStatus";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<crate::types::string::Cid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<crate::types::string::Did>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub subject: crate::types::Union<OutputSubjectRefs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub takedown: Option<crate::com::atproto::admin::defs::StatusAttr>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum OutputSubjectRefs {
    #[serde(rename = "com.atproto.admin.defs#repoRef")]
    ComAtprotoAdminDefsRepoRef(Box<crate::com::atproto::admin::defs::RepoRef>),
    #[serde(rename = "com.atproto.repo.strongRef")]
    ComAtprotoRepoStrongRefMain(Box<crate::com::atproto::repo::strong_ref::Main>),
    #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
    ComAtprotoAdminDefsRepoBlobRef(Box<crate::com::atproto::admin::defs::RepoBlobRef>),
}
