use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::application::dto::{AuthResponseDto, LoginDto, RegisterDto};
use crate::application::services::{PasswordHasher, TokenService};
use crate::domain::entities::{User, UserRole};
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;

pub struct AuthUseCase {
    user_repo: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
}

impl AuthUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self {
            user_repo,
            hasher,
            token_service,
        }
    }

    pub async fn register(&self, dto: RegisterDto) -> Result<AuthResponseDto, DomainError> {
        let trimmed_email = dto.email.trim().to_lowercase();
        if trimmed_email.is_empty() || !trimmed_email.contains('@') {
            return Err(DomainError::ValidationFailed("Invalid email format".into()));
        }

        if dto.password.len() < 6 {
            return Err(DomainError::ValidationFailed("Password must be at least 6 characters".into()));
        }

        if (self.user_repo.find_by_email(&trimmed_email).await?).is_some() {
            return Err(DomainError::Conflict("Email already registered".into()));
        }

        let hashed = self.hasher.hash_password(&dto.password)?;
        let role = dto.role.unwrap_or(UserRole::Rider);
        let user = User {
            id: Uuid::new_v4().to_string(),
            email: trimmed_email,
            password_hash: hashed,
            display_name: dto.display_name,
            photo_url: None,
            role: role.clone(),
            created_at: Utc::now(),
        };

        self.user_repo.save(&user).await?;

        let role_str = match user.role {
            UserRole::Rider => "rider",
            UserRole::Driver => "driver",
            UserRole::Admin => "admin",
        };

        let token = self.token_service.generate_token(&user.id, role_str)?;

        Ok(AuthResponseDto { token, user })
    }

    pub async fn login(&self, dto: LoginDto) -> Result<AuthResponseDto, DomainError> {
        let trimmed_email = dto.email.trim().to_lowercase();
        let user = self
            .user_repo
            .find_by_email(&trimmed_email)
            .await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid email or password".into()))?;

        let is_valid = self.hasher.verify_password(&dto.password, &user.password_hash)?;
        if !is_valid {
            return Err(DomainError::Unauthorized("Invalid email or password".into()));
        }

        let role_str = match user.role {
            UserRole::Rider => "rider",
            UserRole::Driver => "driver",
            UserRole::Admin => "admin",
        };

        let token = self.token_service.generate_token(&user.id, role_str)?;

        Ok(AuthResponseDto { token, user })
    }

    pub async fn get_user_profile(&self, user_id: &str) -> Result<User, DomainError> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".into()))
    }
}
