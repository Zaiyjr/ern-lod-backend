use async_trait::async_trait;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use crate::domain::entities::{Ride, RideStatus};
use crate::domain::errors::DomainError;
use crate::domain::repositories::RideRepository;

#[derive(Clone)]
pub struct InMemoryRideRepository {
    rides: Arc<RwLock<HashMap<String, Ride>>>,
}

impl InMemoryRideRepository {
    pub fn new() -> Self {
        Self {
            rides: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryRideRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RideRepository for InMemoryRideRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Ride>, DomainError> {
        let lock = self.rides.read();
        Ok(lock.get(id).cloned())
    }

    async fn save(&self, ride: &Ride) -> Result<(), DomainError> {
        let mut lock = self.rides.write();
        lock.insert(ride.id.clone(), ride.clone());
        Ok(())
    }

    async fn find_active_by_rider_id(&self, rider_id: &str) -> Result<Option<Ride>, DomainError> {
        let lock = self.rides.read();
        for ride in lock.values() {
            if ride.rider_id == rider_id
                && (ride.status == RideStatus::Requested
                    || ride.status == RideStatus::DriverAssigned
                    || ride.status == RideStatus::ArrivedAtPickup
                    || ride.status == RideStatus::InProgress)
            {
                return Ok(Some(ride.clone()));
            }
        }
        Ok(None)
    }

    async fn find_active_by_driver_id(&self, driver_id: &str) -> Result<Option<Ride>, DomainError> {
        let lock = self.rides.read();
        for ride in lock.values() {
            if ride.driver_id.as_deref() == Some(driver_id)
                && (ride.status == RideStatus::DriverAssigned
                    || ride.status == RideStatus::ArrivedAtPickup
                    || ride.status == RideStatus::InProgress)
            {
                return Ok(Some(ride.clone()));
            }
        }
        Ok(None)
    }

    async fn list_pending_requests(&self) -> Result<Vec<Ride>, DomainError> {
        let lock = self.rides.read();
        let pending = lock
            .values()
            .filter(|r| r.status == RideStatus::Requested)
            .cloned()
            .collect();
        Ok(pending)
    }
}
