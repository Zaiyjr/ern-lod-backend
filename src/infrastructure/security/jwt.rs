use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::application::services::{Claims, TokenService};
use crate::domain::errors::DomainError;

pub struct JwtTokenService {
    secret: Vec<u8>,
}

impl JwtTokenService {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.as_bytes().to_vec(),
        }
    }
}

#[async_trait]
impl TokenService for JwtTokenService {
    fn generate_token(&self, user_id: &str, role: &str) -> Result<String, DomainError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(7))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|e| DomainError::InfrastructureError(format!("JWT generation failed: {}", e)))
    }

    fn verify_token(&self, token: &str) -> Result<Claims, DomainError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| DomainError::Unauthorized(format!("Invalid or expired token: {}", e)))
    }
}
