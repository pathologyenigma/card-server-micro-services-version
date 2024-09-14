use std::{borrow::BorrowMut, time::Duration};

use diesel::{ConnectionError, ConnectionResult};
use diesel_async::{
    pooled_connection::{
        bb8::{Pool, PooledConnection},
        AsyncDieselConnectionManager, ManagerConfig,
    },
    AsyncPgConnection,
};
use futures_util::{future::BoxFuture, FutureExt};
use token_records::TokenRecords;
use users::{NewUser, UpdateUser, User};


pub mod users;
pub mod token_records;

async fn get_conncetion_pool() -> Pool<AsyncPgConnection> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(establish_connection);
    let mgr =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(database_url, config);
    Pool::builder()
        .max_size(10)
        .min_idle(Some(5))
        .max_lifetime(Some(Duration::from_secs(60 * 60 * 24)))
        .idle_timeout(Some(Duration::from_secs(60 * 2)))
        .build(mgr)
        .await
        .expect("Failed to create connection pool")
}

fn establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        let rustls_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_certs())
            .with_no_client_auth();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });
        AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

fn root_certs() -> rustls::RootCertStore {
    let mut roots = rustls::RootCertStore::empty();
    let certs =
        rustls_native_certs::load_native_certs().expect("could not load platform certificates");
    roots.add_parsable_certificates(certs);
    roots
}
#[async_trait::async_trait]
pub trait Repository {
    async fn new() -> Self;
    async fn get_connection(&self) -> PooledConnection<AsyncPgConnection>;
}

pub struct UserRepository {
    conn: Pool<AsyncPgConnection>,
}
#[async_trait::async_trait]
impl Repository for UserRepository {
    async fn new() -> Self {
        Self {
            conn: get_conncetion_pool().await,
        }
    }

    async fn get_connection(&self) -> PooledConnection<AsyncPgConnection> {
        self.conn
            .get()
            .await
            .expect("Failed to get connection from pool")
    }
}
impl UserRepository {

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, tonic::Status> {
        new_user
            .insert(&mut self.get_connection().await.borrow_mut())
            .await
            .map_err(|e| match e {
                diesel::result::Error::InvalidCString(_) => {
                    tonic::Status::invalid_argument("Invalid string")
                }
                diesel::result::Error::DatabaseError(kind, msg) => match kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        tonic::Status::already_exists(msg.message())
                    }
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                        tonic::Status::not_found(msg.message())
                    }
                    diesel::result::DatabaseErrorKind::NotNullViolation => {
                        tonic::Status::invalid_argument(msg.message())
                    }
                    diesel::result::DatabaseErrorKind::CheckViolation => {
                        tonic::Status::invalid_argument(msg.message())
                    }

                    _ => tonic::Status::internal(msg.message()),
                },
                diesel::result::Error::NotFound => tonic::Status::not_found(e.to_string()),
                _ => tonic::Status::internal(e.to_string()),
            })
    }

    pub async fn find_user_by_id(&self, user_id: uuid::Uuid) -> Result<Option<User>, tonic::Status> {
        User::find_by_id(user_id, &mut self.get_connection().await.borrow_mut())
            .await
            .map_err(|e| match e {
                 diesel::result::Error::NotFound => tonic::Status::not_found(e.to_string()),
                 _ => tonic::Status::internal(e.to_string()),
            })
    }

    pub async fn find_user_by_email(&self, email: &str) -> Option<User> {
        User::find_by_email(email, &mut self.get_connection().await.borrow_mut())
            .await
            .expect("failed to find user by email")
    }

    pub async fn find_user_by_text_search(&self, query: &str) -> Result<Vec<User>, tonic::Status> {
        User::text_search(query, &mut self.get_connection().await.borrow_mut())
            .await
            .map_err(|e| match e {
                 diesel::result::Error::NotFound => tonic::Status::not_found(e.to_string()),
                 _ => tonic::Status::internal(e.to_string()),
            })
    }
    pub async fn update_user(&self, user_id: uuid::Uuid, update_user: UpdateUser) -> Result<i32, tonic::Status> {
        User::update(user_id, update_user, &mut self.get_connection().await.borrow_mut()).await.map_err(|e| {
            match e {
                diesel::NotFound => tonic::Status::not_found(e.to_string()),
                _ => tonic::Status::internal(e.to_string()),
            }
        }).map(|_| 1)
    }
    pub async fn delete_user(&self, user_id: uuid::Uuid) -> Result<i32, tonic::Status> {
        User::delete(user_id, &mut self.get_connection().await.borrow_mut()).await.map(|_| 1).map_err(|e| {
            match e {
                diesel::NotFound => tonic::Status::not_found(e.to_string()),
                _ => tonic::Status::internal(e.to_string()),
            }
        })
    }
    
}
pub struct TokenRecordRepository {
    conn: Pool<AsyncPgConnection>,
}

#[async_trait::async_trait]
impl Repository for TokenRecordRepository {
    async fn new() -> Self {
        Self {
            conn: get_conncetion_pool().await,
        }
    }
    async fn get_connection(&self) -> PooledConnection<AsyncPgConnection> {
        self.conn
            .get()
            .await
            .expect("Failed to get connection from pool")
    }
}

impl TokenRecordRepository {
    pub async fn create_token_record(&self, token_record: token_records::TokenRecords) -> Result<token_records::TokenRecords, tonic::Status> {
        token_record
            .insert_new_token_record(&mut self.get_connection().await.borrow_mut())
            .await
            .map_err(|e| match e {
                diesel::result::Error::InvalidCString(_) => {
                    tonic::Status::invalid_argument("Invalid string")
                }
                diesel::result::Error::DatabaseError(kind, msg) => match kind {
                    _ => tonic::Status::internal(msg.message()),
                },
                diesel::result::Error::NotFound => tonic::Status::not_found(e.to_string()),
                _ => tonic::Status::internal(e.to_string()),
            })
    }

    pub async fn check_token_exists_by_its_value(
        &self,
        token_value: &str,
    ) -> Result<bool, tonic::Status> {
        TokenRecords::check_token_exists_by_its_value(
            token_value.to_owned(),
            &mut self.get_connection().await.borrow_mut(),
        ).await.map_err(|err| {
            match err {
                diesel::result::Error::InvalidCString(_) => {
                    tonic::Status::invalid_argument("Invalid string")
                }
                diesel::result::Error::DatabaseError(kind, msg) => match kind {
                    _ => tonic::Status::internal(msg.message()),
                },
                diesel::result::Error::NotFound => tonic::Status::not_found(err.to_string()),
                _ => tonic::Status::internal(err.to_string()),
            }
        })

    }
}
