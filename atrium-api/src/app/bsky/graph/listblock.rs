// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.graph.listblock` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordData {
    pub created_at: crate::types::string::Datetime,
    ///Reference (AT-URI) to the mod list record.
    pub subject: String,
}
pub type Record = crate::types::Object<RecordData>;
