mod traits {
    tonic::include_proto!("user");
    use crate::repository::{
        models::{
            token_records::TokenRecords,
            users::{NewUser, UpdateUser},
        },
        schema::sql_types::InvalidTokenReasonEnum,
    };

    use super::UserService as UserServiceImpl;
    use tonic::Response;
    use user_service_server::*;
    #[tonic::async_trait]
    impl UserService for UserServiceImpl {
        async fn get_user_info(
            &self,
            request: tonic::Request<GetUserRequest>,
        ) -> Result<Response<GetUserResponse>, tonic::Status> {
            let user_id = request.into_inner().user_id;
            let user_id = uuid::Uuid::parse_str(&user_id);
            if let Ok(id) = user_id {
                match self.user_repository.find_user_by_id(id).await? {
                    Some(user) => Ok(Response::new(GetUserResponse {
                        user_id: user.id.to_string(),
                        name: user.name,
                        email: user.email,
                        image: user.image,
                        description: user.description,
                    })),
                    None => Err(tonic::Status::not_found("User not found")),
                }
            } else {
                return Err(tonic::Status::invalid_argument("Invalid user id"));
            }
        }
        async fn create_user(
            &self,
            request: tonic::Request<CreateUserRequest>,
        ) -> Result<Response<CreateUserResponse>, tonic::Status> {
            let user = request.into_inner();
            let id = uuid::Uuid::new_v4();
            self.user_repository
                .create_user(NewUser::new(id, user.name, user.email, user.password))
                .await
                .map(|user| {
                    Response::new(CreateUserResponse {
                        user_id: user.id.to_string(),
                    })
                })
        }
        async fn update_user(
            &self,
            request: tonic::Request<UpdateUserRequest>,
        ) -> Result<Response<UpdateUserResponse>, tonic::Status> {
            let user = request.into_inner();
            let user_id = uuid::Uuid::parse_str(&user.user_id);
            if let Ok(id) = user_id {
                Ok(Response::new(UpdateUserResponse {
                    updated_count: self
                        .user_repository
                        .update_user(
                            id,
                            UpdateUser::new(
                                user.name,
                                user.email,
                                user.password,
                                user.image,
                                user.description,
                            ),
                        )
                        .await?,
                }))
            } else {
                return Err(tonic::Status::invalid_argument("Invalid user id"));
            }
        }
        async fn delete_user(
            &self,
            request: tonic::Request<DeleteUserRequest>,
        ) -> Result<Response<DeleteUserResponse>, tonic::Status> {
            let user_id = request.into_inner().user_id;
            let user_id = uuid::Uuid::parse_str(&user_id);
            if let Ok(id) = user_id {
                Ok(Response::new(DeleteUserResponse {
                    deleted_count: self.user_repository.delete_user(id).await?,
                }))
            } else {
                return Err(tonic::Status::invalid_argument("Invalid user id"));
            }
        }
        async fn login(
            &self,
            request: tonic::Request<LoginRequest>,
        ) -> Result<Response<LoginResponse>, tonic::Status> {
            let user = request.into_inner();
            match self.user_repository.find_user_by_email(&user.email).await {
                Some(user) => {
                    if user.password == user.password {
                        Ok(Response::new(LoginResponse {
                            user_id: user.id.to_string(),
                        }))
                    } else {
                        Err(tonic::Status::unauthenticated("Invalid password"))
                    }
                }
                None => Err(tonic::Status::not_found("User not found")),
            }
        }
        async fn logout(
            &self,
            request: tonic::Request<LogoutRequest>,
        ) -> Result<Response<LogoutResponse>, tonic::Status> {
            let token = request.into_inner().token;
            if self
                .invalid_token_repository
                .check_token_exists_by_its_value(&token)
                .await?
            {
                return Err(tonic::Status::unauthenticated("Invalid token"));
            } else {
                self.invalid_token_repository
                    .create_token_record(TokenRecords {
                        id: uuid::Uuid::new_v4(),
                        token_value: token,
                        invalid_reason: InvalidTokenReasonEnum::AlreadyLogOut,
                    })
                    .await
                    .map(|_| {
                        Response::new(LogoutResponse {
                            timestamp: chrono::Utc::now().timestamp().to_string(),
                        })
                    })
            }
        }
        async fn search_user(
            &self,
            request: tonic::Request<SearchUserRequest>,
        ) -> Result<Response<SearchUserResponse>, tonic::Status> {
            let query = request.into_inner().query;
            let users = self.user_repository.find_user_by_text_search(&query).await?;
            Ok(Response::new(SearchUserResponse {
                user_ids: users.iter().map(|user| user.id.to_string()).collect(),
            }))
        }
    }
}

use crate::repository::models::TokenRecordRepository;

use super::repository::models::UserRepository;

pub struct UserService {
    user_repository: UserRepository,
    invalid_token_repository: TokenRecordRepository,
}
