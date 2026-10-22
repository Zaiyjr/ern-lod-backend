use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::application::dto::{RequestRideDto, UpdateRideStatusDto};
use crate::domain::entities::{Ride, RideStatus};
use crate::domain::errors::DomainError;
use crate::domain::repositories::{RideRepository, UserRepository};

pub struct RideUseCase {
    ride_repo: Arc<dyn RideRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl RideUseCase {
    pub fn new(ride_repo: Arc<dyn RideRepository>, user_repo: Arc<dyn UserRepository>) -> Self {
        Self { ride_repo, user_repo }
    }

    /// Calculate distance in km using Haversine formula
    fn calculate_distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6371.0; // Earth radius in km
        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();

        let a = (d_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        r * c
    }

    /// Calculate estimated fare based on distance: base fare + rate per km
    fn estimate_fare(distance_km: f64) -> f64 {
        let base_fare = 2.50;
        let rate_per_km = 1.20;
        let fare = base_fare + (distance_km * rate_per_km);
        (fare * 100.0).round() / 100.0
    }

    pub async fn request_ride(&self, rider_id: &str, dto: RequestRideDto) -> Result<Ride, DomainError> {
        // Check if rider already has an active ride
        if let Some(active) = self.ride_repo.find_active_by_rider_id(rider_id).await? {
            return Err(DomainError::Conflict(format!(
                "Rider already has an active ride: {}",
                active.id
            )));
        }

        let distance = Self::calculate_distance_km(
            dto.pickup.latitude,
            dto.pickup.longitude,
            dto.destination.latitude,
            dto.destination.longitude,
        );

        let estimated_fare = Self::estimate_fare(distance);

        let ride = Ride {
            id: Uuid::new_v4().to_string(),
            rider_id: rider_id.to_string(),
            driver_id: None,
            status: RideStatus::Requested,
            pickup: dto.pickup,
            destination: dto.destination,
            estimated_fare,
            actual_fare: None,
            requested_at: Utc::now(),
            matched_at: None,
            completed_at: None,
        };

        self.ride_repo.save(&ride).await?;
        Ok(ride)
    }

    pub async fn match_nearest_driver(&self, ride_id: &str) -> Result<Ride, DomainError> {
        let mut ride = self
            .ride_repo
            .find_by_id(ride_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Ride not found".into()))?;

        if ride.status != RideStatus::Requested {
            return Err(DomainError::Conflict(
                "Ride is not in requested state".into(),
            ));
        }

        let available_drivers = self.user_repo.list_available_drivers().await?;
        let selected_driver = available_drivers
            .first()
            .ok_or_else(|| DomainError::NotFound("No drivers available nearby".into()))?;

        ride.driver_id = Some(selected_driver.id.clone());
        ride.status = RideStatus::DriverAssigned;
        ride.matched_at = Some(Utc::now());

        self.ride_repo.save(&ride).await?;
        Ok(ride)
    }

    pub async fn update_status(
        &self,
        ride_id: &str,
        user_id: &str,
        dto: UpdateRideStatusDto,
    ) -> Result<Ride, DomainError> {
        let mut ride = self
            .ride_repo
            .find_by_id(ride_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Ride not found".into()))?;

        // Verify participant is rider or driver
        let is_driver = ride.driver_id.as_deref() == Some(user_id);
        let is_rider = ride.rider_id == user_id;

        if !is_driver && !is_rider {
            return Err(DomainError::Forbidden(
                "You are not authorized to update this ride".into(),
            ));
        }

        ride.status = dto.status;
        if dto.status == RideStatus::Completed {
            ride.completed_at = Some(Utc::now());
            ride.actual_fare = dto.actual_fare.or(Some(ride.estimated_fare));
        }

        self.ride_repo.save(&ride).await?;
        Ok(ride)
    }

    pub async fn get_ride_by_id(&self, ride_id: &str) -> Result<Ride, DomainError> {
        self.ride_repo
            .find_by_id(ride_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Ride not found".into()))
    }

    pub async fn list_pending_rides(&self) -> Result<Vec<Ride>, DomainError> {
        self.ride_repo.list_pending_requests().await
    }
}
