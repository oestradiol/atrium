// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `tools.ozone.moderation.emitEvent` namespace.
pub const NSID: &str = "tools.ozone.moderation.emitEvent";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub created_by: crate::types::string::Did,
    pub event: crate::types::Union<InputEventRefs>,
    pub subject: crate::types::Union<InputSubjectRefs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_blob_cids: Option<Vec<crate::types::string::Cid>>,
}
pub type Output = crate::tools::ozone::moderation::defs::ModEventView;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    SubjectHasAction(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::SubjectHasAction(msg) => {
                write!(_f, "SubjectHasAction")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum InputEventRefs {
    #[serde(rename = "tools.ozone.moderation.defs#modEventTakedown")]
    ToolsOzoneModerationDefsModEventTakedown(
        Box<crate::tools::ozone::moderation::defs::ModEventTakedown>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventAcknowledge")]
    ToolsOzoneModerationDefsModEventAcknowledge(
        Box<crate::tools::ozone::moderation::defs::ModEventAcknowledge>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventEscalate")]
    ToolsOzoneModerationDefsModEventEscalate(
        Box<crate::tools::ozone::moderation::defs::ModEventEscalate>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventComment")]
    ToolsOzoneModerationDefsModEventComment(
        Box<crate::tools::ozone::moderation::defs::ModEventComment>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventLabel")]
    ToolsOzoneModerationDefsModEventLabel(
        Box<crate::tools::ozone::moderation::defs::ModEventLabel>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventReport")]
    ToolsOzoneModerationDefsModEventReport(
        Box<crate::tools::ozone::moderation::defs::ModEventReport>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventMute")]
    ToolsOzoneModerationDefsModEventMute(
        Box<crate::tools::ozone::moderation::defs::ModEventMute>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventUnmute")]
    ToolsOzoneModerationDefsModEventUnmute(
        Box<crate::tools::ozone::moderation::defs::ModEventUnmute>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventMuteReporter")]
    ToolsOzoneModerationDefsModEventMuteReporter(
        Box<crate::tools::ozone::moderation::defs::ModEventMuteReporter>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventUnmuteReporter")]
    ToolsOzoneModerationDefsModEventUnmuteReporter(
        Box<crate::tools::ozone::moderation::defs::ModEventUnmuteReporter>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventReverseTakedown")]
    ToolsOzoneModerationDefsModEventReverseTakedown(
        Box<crate::tools::ozone::moderation::defs::ModEventReverseTakedown>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventEmail")]
    ToolsOzoneModerationDefsModEventEmail(
        Box<crate::tools::ozone::moderation::defs::ModEventEmail>,
    ),
    #[serde(rename = "tools.ozone.moderation.defs#modEventTag")]
    ToolsOzoneModerationDefsModEventTag(
        Box<crate::tools::ozone::moderation::defs::ModEventTag>,
    ),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum InputSubjectRefs {
    #[serde(rename = "com.atproto.admin.defs#repoRef")]
    ComAtprotoAdminDefsRepoRef(Box<crate::com::atproto::admin::defs::RepoRef>),
    #[serde(rename = "com.atproto.repo.strongRef")]
    ComAtprotoRepoStrongRefMain(Box<crate::com::atproto::repo::strong_ref::Main>),
}
