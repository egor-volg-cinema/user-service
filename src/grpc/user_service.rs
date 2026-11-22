use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::proto::users::v1::{
    CreateUserRequest, CreateUserResponse, GetMeRequest, GetMeResponse, PatchUserRequest,
    PatchUserResponse, User as ProtoUser, users_service_server::UsersService,
};
use crate::repository::db::{self, Database};

#[derive(Clone)]
pub struct UserService {
    db: Arc<Database>,
}

impl UserService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
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
            .get_user(req.id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("user not found"))?;
        Ok(Response::new(GetMeResponse {
            user: Some(user.into()),
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

impl From<db::User> for ProtoUser {
    fn from(user: db::User) -> Self {
        Self {
            id: user.id.to_string(),
            name: Some(user.name),
            phone: None,
            email: None,
            avatar: Some(user.avatar),
        }
    }
}
