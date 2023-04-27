// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.getRepo` namespace.

/// View details about a repository.
#[async_trait::async_trait]
pub trait GetRepo: crate::xrpc::XrpcClient {
    async fn get_repo(&self, params: Parameters) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.admin.getRepo",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub did: String,
}

pub type Output = crate::com::atproto::admin::defs::RepoViewDetail;

pub enum Error {
}
