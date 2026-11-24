use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::grpc::clients::account_service::AccountService;
use crate::proto::users::v1::{
    CreateUserRequest, CreateUserResponse, GetMeRequest, GetMeResponse, PatchUserRequest,
    PatchUserResponse, User as ProtoUser, users_service_server::UsersService,
};
use crate::repository::db::{self, Database};

#[derive(Clone)]
pub struct UserService {
    db: Arc<Database>,
    account: AccountService,
}

impl UserService {
    pub fn new(db: Arc<Database>) -> Result<Self, tonic::transport::Error> {
        let account_url =
            std::env::var("AUTH_GRPC_URL").unwrap_or_else(|_| "localhost:50051".into());
        Ok(Self {
            db,
            account: AccountService::new(&account_url)?,
        })
    }
}

impl From<db::User> for ProtoUser {
    fn from(u: db::User) -> Self {
        Self {
            id: u.id,
            name: non_empty(u.name),
            avatar: non_empty(u.avatar),
            phone: None,
            email: None,
        }
    }
}

fn non_empty(s: String) -> Option<String> {
    (!s.is_empty()).then_some(s)
}

#[tonic::async_trait]
impl UsersService for UserService {
    async fn get_me(
        &self,
        request: Request<GetMeRequest>,
    ) -> Result<Response<GetMeResponse>, Status> {
        let req = request.into_inner();
        let user = self
            .db
            .get_user(req.id.clone())
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("user not found"))?;

        let account = self
            .account
            .get_account(&req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut user: ProtoUser = user.into();
        user.phone = non_empty(account.phone);
        user.email = non_empty(account.email);

        Ok(Response::new(GetMeResponse {
            user: Some(user),
        }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        self.db
            .create_user(db::User::new(req.id, String::new(), String::new()))
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateUserResponse { ok: true }))
    }

    async fn patch_user(
        &self,
        request: Request<PatchUserRequest>,
    ) -> Result<Response<PatchUserResponse>, Status> {
        let req = request.into_inner();

        self.db
            .update_user(req.user_id, req.name, req.avatar)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(PatchUserResponse { ok: true }))
    }
}
