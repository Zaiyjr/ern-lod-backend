-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(64) PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    photo_url TEXT,
    role VARCHAR(32) NOT NULL DEFAULT 'rider',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index on email for fast lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Create rides table
CREATE TABLE IF NOT EXISTS rides (
    id VARCHAR(64) PRIMARY KEY,
    rider_id VARCHAR(64) NOT NULL REFERENCES users(id),
    driver_id VARCHAR(64) REFERENCES users(id),
    status VARCHAR(32) NOT NULL,
    pickup_latitude DOUBLE PRECISION NOT NULL,
    pickup_longitude DOUBLE PRECISION NOT NULL,
    pickup_address TEXT,
    destination_latitude DOUBLE PRECISION NOT NULL,
    destination_longitude DOUBLE PRECISION NOT NULL,
    destination_address TEXT,
    estimated_fare DOUBLE PRECISION NOT NULL,
    actual_fare DOUBLE PRECISION,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    matched_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);

-- Index on active rides queries
CREATE INDEX IF NOT EXISTS idx_rides_rider_status ON rides(rider_id, status);
CREATE INDEX IF NOT EXISTS idx_rides_driver_status ON rides(driver_id, status);
