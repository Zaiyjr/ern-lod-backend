use async_trait::async_trait;
use crate::domain::entities::Ride;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait RideRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Ride>, DomainError>;
    async fn save(&self, ride: &Ride) -> Result<(), DomainError>;
    async fn find_active_by_rider_id(&self, rider_id: &str) -> Result<Option<Ride>, DomainError>;
    async fn find_active_by_driver_id(&self, driver_id: &str) -> Result<Option<Ride>, DomainError>;
    async fn list_pending_requests(&self) -> Result<Vec<Ride>, DomainError>;
}
