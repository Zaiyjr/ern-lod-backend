use async_trait::async_trait;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use crate::domain::entities::{User, UserRole};
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;

#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DomainError> {
        let lock = self.users.read();
        Ok(lock.get(id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let lock = self.users.read();
        for user in lock.values() {
            if user.email.eq_ignore_ascii_case(email) {
                return Ok(Some(user.clone()));
            }
        }
        Ok(None)
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut lock = self.users.write();
        lock.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn list_available_drivers(&self) -> Result<Vec<User>, DomainError> {
        let lock = self.users.read();
        let drivers = lock
            .values()
            .filter(|u| u.role == UserRole::Driver)
            .cloned()
            .collect();
        Ok(drivers)
    }
}
