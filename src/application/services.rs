use async_trait::async_trait;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, DomainError>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, DomainError>;
}

#[async_trait]
pub trait TokenService: Send + Sync {
    fn generate_token(&self, user_id: &str, role: &str) -> Result<String, DomainError>;
    fn verify_token(&self, token: &str) -> Result<Claims, DomainError>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}
