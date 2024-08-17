mod traits {
    tonic::include_proto!("user");
    use crate::repository::models::users::{NewUser, User};

    use super::UserService as UserServiceImpl;
    use tonic::{IntoRequest, Response};
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
                match self.repository.find_user_by_id(id).await {
                    Some(user) => Ok(Response::new(GetUserResponse {
                        user_id: user.id.to_string(),
                        name: user.name,
                        email: user.email,
                        image: user.image,
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
            self.repository
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
                match self.repository.find_user_by_id(id).await {
                    Some(mut u) => {
                        if let Some(name) = user.name {
                            u.name = name;
                        }
                        if let Some(email) = user.email {
                            u.email = email;
                        }
                        if let Some(image) = user.image {
                            u.image = Some(image);
                        }
                        self.repository.update_user(u).await?;
                        Ok(Response::new(UpdateUserResponse {
                            result: true,
                        }))
                    }
                    None => Err(tonic::Status::not_found("User not found")),
                }
            } else {
                return Err(tonic::Status::invalid_argument("Invalid user id"));
            }
        }
    }
}

use super::repository::models::UserRepository;

pub struct UserService {
    repository: UserRepository,
}
