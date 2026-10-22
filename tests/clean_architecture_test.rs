#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use backend::application::dto::{LoginDto, RegisterDto, RequestRideDto, UpdateRideStatusDto};
    use backend::application::services::{PasswordHasher, TokenService};
    use backend::application::usecases::{AuthUseCase, RideUseCase};
    use backend::domain::entities::{Location, RideStatus, UserRole};
    use backend::infrastructure::repositories::{InMemoryRideRepository, InMemoryUserRepository};
    use backend::infrastructure::security::{Argon2PasswordHasher, JwtTokenService};

    fn setup() -> (AuthUseCase, RideUseCase) {
        let user_repo = Arc::new(InMemoryUserRepository::new());
        let ride_repo = Arc::new(InMemoryRideRepository::new());
        let hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2PasswordHasher::new());
        let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new("test-secret-key-1234567890"));

        let auth_uc = AuthUseCase::new(user_repo.clone(), hasher, token_service);
        let ride_uc = RideUseCase::new(ride_repo, user_repo);

        (auth_uc, ride_uc)
    }

    #[tokio::test]
    async fn test_register_and_login_flow() {
        let (auth_uc, _) = setup();

        let register_res = auth_uc
            .register(RegisterDto {
                email: "rider@example.com".into(),
                password: "password123".into(),
                display_name: Some("Test Rider".into()),
                role: Some(UserRole::Rider),
            })
            .await
            .expect("Registration should succeed");

        assert_eq!(register_res.user.email, "rider@example.com");
        assert!(!register_res.token.is_empty());

        let login_res = auth_uc
            .login(LoginDto {
                email: "rider@example.com".into(),
                password: "password123".into(),
            })
            .await
            .expect("Login should succeed");

        assert_eq!(login_res.user.id, register_res.user.id);
        assert!(!login_res.token.is_empty());
    }

    #[tokio::test]
    async fn test_ride_booking_lifecycle() {
        let (auth_uc, ride_uc) = setup();

        // 1. Register rider and driver
        let rider = auth_uc
            .register(RegisterDto {
                email: "rider1@example.com".into(),
                password: "password123".into(),
                display_name: Some("Rider One".into()),
                role: Some(UserRole::Rider),
            })
            .await
            .unwrap()
            .user;

        let driver = auth_uc
            .register(RegisterDto {
                email: "driver1@example.com".into(),
                password: "password123".into(),
                display_name: Some("Driver One".into()),
                role: Some(UserRole::Driver),
            })
            .await
            .unwrap()
            .user;

        // 2. Rider requests ride
        let ride = ride_uc
            .request_ride(
                &rider.id,
                RequestRideDto {
                    pickup: Location {
                        latitude: 13.7563,
                        longitude: 100.5018,
                        address: Some("Bangkok Center".into()),
                    },
                    destination: Location {
                        latitude: 13.7460,
                        longitude: 100.5340,
                        address: Some("Siam Paragon".into()),
                    },
                },
            )
            .await
            .expect("Ride request should succeed");

        assert_eq!(ride.status, RideStatus::Requested);
        assert!(ride.estimated_fare > 0.0);

        // 3. Backend matches driver
        let matched = ride_uc
            .match_nearest_driver(&ride.id)
            .await
            .expect("Matching driver should succeed");

        assert_eq!(matched.status, RideStatus::DriverAssigned);
        assert_eq!(matched.driver_id, Some(driver.id.clone()));

        // 4. Driver completes trip
        let completed = ride_uc
            .update_status(
                &ride.id,
                &driver.id,
                UpdateRideStatusDto {
                    status: RideStatus::Completed,
                    actual_fare: Some(7.50),
                },
            )
            .await
            .expect("Completing ride should succeed");

        assert_eq!(completed.status, RideStatus::Completed);
        assert_eq!(completed.actual_fare, Some(7.50));
        assert!(completed.completed_at.is_some());
    }
}
