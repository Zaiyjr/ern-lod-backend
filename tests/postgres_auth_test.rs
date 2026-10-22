#[cfg(test)]
mod postgres_tests {
    use std::sync::Arc;
    use backend::application::dto::{LoginDto, RegisterDto};
    use backend::application::services::{PasswordHasher, TokenService};
    use backend::application::usecases::AuthUseCase;
    use backend::domain::entities::UserRole;
    use backend::infrastructure::repositories::PostgresUserRepository;
    use backend::infrastructure::security::{Argon2PasswordHasher, JwtTokenService};
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_postgres_auth_register_and_login() {
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgrespassword@localhost:5432/ride_booking_db".to_string());

        let pool = match PgPoolOptions::new().max_connections(5).connect(&db_url).await {
            Ok(p) => p,
            Err(_) => {
                println!("Skipping postgres test: cannot connect to DB at {}", db_url);
                return;
            }
        };

        // Ensure schema exists
        sqlx::raw_sql(include_str!("../migrations/0001_init.sql"))
            .execute(&pool)
            .await
            .expect("Failed to apply migration in test");

        let user_repo = Arc::new(PostgresUserRepository::new(pool));
        let hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2PasswordHasher::new());
        let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new("test-secret-key-1234567890"));
        let auth_uc = AuthUseCase::new(user_repo, hasher, token_service);

        let unique_email = format!("driver_{}@test.com", Uuid::new_v4());

        // 1. Register with Postgres
        let reg_res = auth_uc
            .register(RegisterDto {
                email: unique_email.clone(),
                password: "secure_pass_123".into(),
                display_name: Some("Somchai Driver".into()),
                role: Some(UserRole::Driver),
            })
            .await
            .expect("Registration to Postgres should succeed");

        assert_eq!(reg_res.user.email, unique_email);
        assert_eq!(reg_res.user.role, UserRole::Driver);
        assert!(!reg_res.token.is_empty());

        // 2. Login with Postgres
        let login_res = auth_uc
            .login(LoginDto {
                email: unique_email,
                password: "secure_pass_123".into(),
            })
            .await
            .expect("Login to Postgres should succeed");

        assert_eq!(login_res.user.id, reg_res.user.id);
        assert!(!login_res.token.is_empty());
    }
}
