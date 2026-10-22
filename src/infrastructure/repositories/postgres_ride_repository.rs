use async_trait::async_trait;
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};
use crate::domain::entities::{Location, Ride, RideStatus};
use crate::domain::errors::DomainError;
use crate::domain::repositories::RideRepository;

pub struct PostgresRideRepository {
    pool: PgPool,
}

impl PostgresRideRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RideRepository for PostgresRideRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Ride>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, rider_id, driver_id, status,
                   pickup_latitude, pickup_longitude, pickup_address,
                   destination_latitude, destination_longitude, destination_address,
                   estimated_fare, actual_fare, requested_at, matched_at, completed_at
            FROM rides
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error in find_by_id: {}", e)))?;

        match row {
            Some(row) => {
                let status_str: String = row.try_get("status").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                let status = match status_str.as_str() {
                    "driver_assigned" => RideStatus::DriverAssigned,
                    "arrived_at_pickup" => RideStatus::ArrivedAtPickup,
                    "in_progress" => RideStatus::InProgress,
                    "completed" => RideStatus::Completed,
                    "cancelled" => RideStatus::Cancelled,
                    _ => RideStatus::Requested,
                };

                let requested_at: DateTime<Utc> = row.try_get("requested_at").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                let matched_at: Option<DateTime<Utc>> = row.try_get("matched_at").ok();
                let completed_at: Option<DateTime<Utc>> = row.try_get("completed_at").ok();

                Ok(Some(Ride {
                    id: row.try_get("id").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    rider_id: row.try_get("rider_id").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    driver_id: row.try_get("driver_id").ok(),
                    status,
                    pickup: Location {
                        latitude: row.try_get("pickup_latitude").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                        longitude: row.try_get("pickup_longitude").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                        address: row.try_get("pickup_address").ok(),
                    },
                    destination: Location {
                        latitude: row.try_get("destination_latitude").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                        longitude: row.try_get("destination_longitude").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                        address: row.try_get("destination_address").ok(),
                    },
                    estimated_fare: row.try_get("estimated_fare").map_err(|e| DomainError::InfrastructureError(e.to_string()))?,
                    actual_fare: row.try_get("actual_fare").ok(),
                    requested_at,
                    matched_at,
                    completed_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, ride: &Ride) -> Result<(), DomainError> {
        let status_str = match ride.status {
            RideStatus::Requested => "requested",
            RideStatus::DriverAssigned => "driver_assigned",
            RideStatus::ArrivedAtPickup => "arrived_at_pickup",
            RideStatus::InProgress => "in_progress",
            RideStatus::Completed => "completed",
            RideStatus::Cancelled => "cancelled",
        };

        sqlx::query(
            r#"
            INSERT INTO rides (
                id, rider_id, driver_id, status,
                pickup_latitude, pickup_longitude, pickup_address,
                destination_latitude, destination_longitude, destination_address,
                estimated_fare, actual_fare, requested_at, matched_at, completed_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            ON CONFLICT (id) DO UPDATE
            SET driver_id = EXCLUDED.driver_id,
                status = EXCLUDED.status,
                actual_fare = EXCLUDED.actual_fare,
                matched_at = EXCLUDED.matched_at,
                completed_at = EXCLUDED.completed_at
            "#,
        )
        .bind(&ride.id)
        .bind(&ride.rider_id)
        .bind(&ride.driver_id)
        .bind(status_str)
        .bind(ride.pickup.latitude)
        .bind(ride.pickup.longitude)
        .bind(&ride.pickup.address)
        .bind(ride.destination.latitude)
        .bind(ride.destination.longitude)
        .bind(&ride.destination.address)
        .bind(ride.estimated_fare)
        .bind(ride.actual_fare)
        .bind(ride.requested_at)
        .bind(ride.matched_at)
        .bind(ride.completed_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Failed to save ride: {}", e)))?;

        Ok(())
    }

    async fn find_active_by_rider_id(&self, rider_id: &str) -> Result<Option<Ride>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id FROM rides
            WHERE rider_id = $1
              AND status IN ('requested', 'driver_assigned', 'arrived_at_pickup', 'in_progress')
            LIMIT 1
            "#,
        )
        .bind(rider_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error in find_active_by_rider_id: {}", e)))?;

        match row {
            Some(r) => {
                let id: String = r.try_get("id").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                self.find_by_id(&id).await
            }
            None => Ok(None),
        }
    }

    async fn find_active_by_driver_id(&self, driver_id: &str) -> Result<Option<Ride>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id FROM rides
            WHERE driver_id = $1
              AND status IN ('driver_assigned', 'arrived_at_pickup', 'in_progress')
            LIMIT 1
            "#,
        )
        .bind(driver_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error in find_active_by_driver_id: {}", e)))?;

        match row {
            Some(r) => {
                let id: String = r.try_get("id").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
                self.find_by_id(&id).await
            }
            None => Ok(None),
        }
    }

    async fn list_pending_requests(&self) -> Result<Vec<Ride>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id FROM rides
            WHERE status = 'requested'
            ORDER BY requested_at ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(format!("Database error in list_pending_requests: {}", e)))?;

        let mut rides = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").map_err(|e| DomainError::InfrastructureError(e.to_string()))?;
            if let Some(ride) = self.find_by_id(&id).await? {
                rides.push(ride);
            }
        }
        Ok(rides)
    }
}
