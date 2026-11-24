use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};

use crate::proto::account::v1::{
    GetAccountRequest, GetAccountResponse, account_service_client::AccountServiceClient,
};

#[derive(Clone)]
pub struct AccountService {
    client: AccountServiceClient<Channel>,
}

impl AccountService {
    pub fn new(url: &str) -> Result<Self, tonic::transport::Error> {
        let url = if url.contains("://") {
            url.to_string()
        } else {
            format!("http://{url}")
        };
        let channel = Endpoint::from_shared(url)?.connect_lazy();
        Ok(Self {
            client: AccountServiceClient::new(channel),
        })
    }

    pub async fn get_account(&self, id: &str) -> Result<GetAccountResponse, Status> {
        self.client
            .clone()
            .get_account(Request::new(GetAccountRequest {
                id: id.to_string(),
            }))
            .await
            .map(|r| r.into_inner())
    }
}
