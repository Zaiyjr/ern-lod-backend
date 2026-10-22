use serde::{Deserialize, Serialize};
use crate::domain::entities::{Location, Ride, RideStatus, User, UserRole};

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponseDto {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct RequestRideDto {
    pub pickup: Location,
    pub destination: Location,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRideStatusDto {
    pub status: RideStatus,
    pub actual_fare: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct RideResponseDto {
    pub ride: Ride,
}
