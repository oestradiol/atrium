// This file is generated by atrium-codegen. Do not edit.
//! Definitions for the `com.atproto.server.createInviteCode` namespace.

/// Create an invite code.
#[async_trait::async_trait]
pub trait CreateInviteCode: crate::xrpc::XrpcClient {
    async fn create_invite_code(&self, input: Input) -> Result<Output, Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "com.atproto.server.createInviteCode",
            Option::<()>::None,
            Some(input),
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_account: Option<String>,
    pub use_count: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub code: String,
}

pub enum Error {
}
