use async_trait::async_trait;
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};
use crate::domain::entities::{User, UserRole};
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, password_hash, display_name, photo_url, role, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error in find_by_id: {}", e)))?;

        match row {
            Some(row) => {
                let role_str: String = row.try_get("role").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                let role = match role_str.as_str() {
                    "driver" => UserRole::Driver,
                    "admin" => UserRole::Admin,
                    _ => UserRole::Rider,
                };

                let created_at: DateTime<Utc> = row.try_get("created_at").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

                Ok(Some(User {
                    id: row.try_get("id").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    email: row.try_get("email").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    password_hash: row.try_get("password_hash").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    display_name: row.try_get("display_name").ok(),
                    photo_url: row.try_get("photo_url").ok(),
                    role,
                    created_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, password_hash, display_name, photo_url, role, created_at
            FROM users
            WHERE LOWER(email) = LOWER($1)
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error in find_by_email: {}", e)))?;

        match row {
            Some(row) => {
                let role_str: String = row.try_get("role").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                let role = match role_str.as_str() {
                    "driver" => UserRole::Driver,
                    "admin" => UserRole::Admin,
                    _ => UserRole::Rider,
                };

                let created_at: DateTime<Utc> = row.try_get("created_at").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

                Ok(Some(User {
                    id: row.try_get("id").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    email: row.try_get("email").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    password_hash: row.try_get("password_hash").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    display_name: row.try_get("display_name").ok(),
                    photo_url: row.try_get("photo_url").ok(),
                    role,
                    created_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let role_str = match user.role {
            UserRole::Rider => "rider",
            UserRole::Driver => "driver",
            UserRole::Admin => "admin",
        };

        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, display_name, photo_url, role, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE
            SET email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                display_name = EXCLUDED.display_name,
                photo_url = EXCLUDED.photo_url,
                role = EXCLUDED.role
            "#,
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.display_name)
        .bind(&user.photo_url)
        .bind(role_str)
        .bind(user.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Failed to save user: {}", e)))?;

        Ok(())
    }

    async fn list_available_drivers(&self) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, email, password_hash, display_name, photo_url, role, created_at
            FROM users
            WHERE role = 'driver'
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Failed to list drivers: {}", e)))?;

        let mut drivers = Vec::new();
        for row in rows {
            let created_at: DateTime<Utc> = row.try_get("created_at").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
            drivers.push(User {
                id: row.try_get("id").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                email: row.try_get("email").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                password_hash: row.try_get("password_hash").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                display_name: row.try_get("display_name").ok(),
                photo_url: row.try_get("photo_url").ok(),
                role: UserRole::Driver,
                created_at,
            });
        }

        Ok(drivers)
    }
}
