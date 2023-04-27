// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.sync.getBlocks` namespace.

/// Gets blocks from a given repo.
#[async_trait::async_trait]
pub trait GetBlocks: crate::xrpc::XrpcClient {
    async fn get_blocks(&self, params: Parameters) -> Result<(), Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::GET,
            "com.atproto.sync.getBlocks",
            Some(params),
            Option::<()>::None,
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub cids: Vec<String>,
    /// The DID of the repo.
    pub did: String,
}


pub enum Error {
}
