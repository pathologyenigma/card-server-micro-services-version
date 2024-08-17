use std::fs;

use poem::http::HeaderMap;
use anyhow::Result;
use serde::Deserialize;
#[derive(serde::Serialize, serde::Deserialize)]
enum Role {
    Admin,
    User,
    Developer,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
    iss: String,
    aud: String,
    jti: uuid::Uuid,
    user_id: uuid::Uuid,
    role: Role
}

fn get_token_from_header(headers: &HeaderMap) -> Option<String> {
    headers.get("Authorization")
       .and_then(|value| value.to_str().ok())
       .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header.split_at(7).1.to_string())
            } else {
                None
            }
        })
}
#[derive(Deserialize)]
pub struct JWTAuthenticator {
    pub(crate) secret_key: String,
    pub(crate) token_header: Option<jsonwebtoken::Header>
}

impl JWTAuthenticator {
    pub fn new() -> Self {
        let s = fs::read_to_string("TokenConfig.toml").expect("Failed to read TokenConfig.toml");
        toml::from_str(s.as_str()).expect("Failed to parse TokenConfig.toml")
    }

    pub fn encode(&self, claims: &Claims) -> Result<String> {
        let token = jsonwebtoken::encode(
            &self.token_header.clone().unwrap_or_default(),
            claims,
            &jsonwebtoken::EncodingKey::from_secret(self.secret_key.as_bytes()),
        )?;
        Ok(token)
    }

    pub fn decode(&self, token: &str) -> Result<Claims> {
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret_key.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )?;
        Ok(claims.claims)
    }

    pub fn authenticate(&self, headers: &HeaderMap) -> Result<Claims> {
        let token = get_token_from_header(headers).ok_or_else(|| anyhow::anyhow!("No token found in headers"))?;
        self.decode(&token)
    }
}