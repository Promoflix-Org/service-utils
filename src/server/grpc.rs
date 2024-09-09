use anyhow::*;
use tonic::transport::Endpoint;

pub mod auth_service {
    tonic::include_proto!("auth_service");
}

use auth_service::{auth_service_client::AuthServiceClient, CheckTokenRequest, CheckTokenResponse};

use crate::AUTH_SERVICE_URL;

pub async fn check_token(
    user_id: &String,
    access_token: &String,
) -> Result<CheckTokenResponse, Error> {
    lazy_static::initialize(&AUTH_SERVICE_URL);
    let endpoint: Endpoint = AUTH_SERVICE_URL.parse().context("Invalid endpoint")?;
    let mut grpc = AuthServiceClient::connect(endpoint)
        .await
        .context("Unable to establish connection with auth service")?;
    let res = grpc
        .check_token(CheckTokenRequest {
            user_id: user_id.to_string(),
            access_token: access_token.to_string(),
        })
        .await
        .context("Unable to send check_token request")?;

    let message = res.into_inner();

    println!("{:?}", message);

    Ok(message)
}
