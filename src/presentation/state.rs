use std::sync::Arc;
use sqlx::PgPool;
use crate::application::services::{PasswordHasher, TokenService};
use crate::application::usecases::{AuthUseCase, RideUseCase};
use crate::domain::repositories::{RideRepository, UserRepository};
use crate::infrastructure::config::Config;
use crate::infrastructure::repositories::{
    InMemoryRideRepository, InMemoryUserRepository, PostgresRideRepository, PostgresUserRepository,
};
use crate::infrastructure::security::{Argon2PasswordHasher, JwtTokenService};

#[derive(Clone)]
pub struct AppState {
    pub auth_usecase: Arc<AuthUseCase>,
    pub ride_usecase: Arc<RideUseCase>,
    pub token_service: Arc<dyn TokenService>,
}

impl AppState {
    pub fn new_in_memory(config: &Config) -> Self {
        let user_repo: Arc<dyn UserRepository> = Arc::new(InMemoryUserRepository::new());
        let ride_repo: Arc<dyn RideRepository> = Arc::new(InMemoryRideRepository::new());
        let hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2PasswordHasher::new());
        let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(&config.jwt_secret));

        let auth_usecase = Arc::new(AuthUseCase::new(
            user_repo.clone(),
            hasher,
            token_service.clone(),
        ));
        let ride_usecase = Arc::new(RideUseCase::new(ride_repo, user_repo));

        Self {
            auth_usecase,
            ride_usecase,
            token_service,
        }
    }

    pub fn new_with_postgres(pool: PgPool, config: &Config) -> Self {
        let user_repo: Arc<dyn UserRepository> = Arc::new(PostgresUserRepository::new(pool.clone()));
        let ride_repo: Arc<dyn RideRepository> = Arc::new(PostgresRideRepository::new(pool));
        let hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2PasswordHasher::new());
        let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(&config.jwt_secret));

        let auth_usecase = Arc::new(AuthUseCase::new(
            user_repo.clone(),
            hasher,
            token_service.clone(),
        ));
        let ride_usecase = Arc::new(RideUseCase::new(ride_repo, user_repo));

        Self {
            auth_usecase,
            ride_usecase,
            token_service,
        }
    }
}
