// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.labeler` namespace.
pub mod defs;
pub mod get_services;
pub mod service;
#[derive(Debug)]
pub struct Service;
impl crate::types::Collection for Service {
    const NSID: &'static str = "app.bsky.labeler.service";
    type Record = service::Record;
}
