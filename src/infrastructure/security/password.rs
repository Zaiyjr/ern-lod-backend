use argon2::{
    password_hash::{PasswordHash, PasswordHasher as _, PasswordVerifier as _, SaltString},
    Argon2,
};
use rand_core::OsRng;
use async_trait::async_trait;
use crate::application::services::PasswordHasher;
use crate::domain::errors::DomainError;

pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| DomainError::InfrastructureError(format!("Argon2 hash error: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| DomainError::InfrastructureError(format!("Argon2 parse error: {}", e)))?;
        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
