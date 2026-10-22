use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RideStatus {
    Requested,
    DriverAssigned,
    ArrivedAtPickup,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ride {
    pub id: String,
    pub rider_id: String,
    pub driver_id: Option<String>,
    pub status: RideStatus,
    pub pickup: Location,
    pub destination: Location,
    pub estimated_fare: f64,
    pub actual_fare: Option<f64>,
    pub requested_at: chrono::DateTime<chrono::Utc>,
    pub matched_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}
