pub struct Config {
    pub port: u16,
    pub jwt_secret: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "super-secret-jwt-ride-booking-key-32b!".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgrespassword@localhost:5433/ride_booking_db".to_string());

        Self {
            port,
            jwt_secret,
            database_url,
        }
    }
}
