# Ride Booking Backend (Rust Clean Architecture + PostgreSQL)

A robust backend service built with **Rust**, **Axum**, **Tokio**, and **SQLx (PostgreSQL)** following Uncle Bob's **Clean Architecture** principles.

---

## 🐳 Local Database Setup (Docker)

A `docker-compose.yml` file is provided to start PostgreSQL 16:

```bash
cd backend
docker compose up -d
```

- **Container:** `ride_booking_postgres`
- **Host / Port:** `localhost:5432`
- **Database:** `ride_booking_db`
- **Username:** `postgres`
- **Password:** `postgrespassword`

To inspect the database:
```bash
docker exec -it ride_booking_postgres psql -U postgres -d ride_booking_db
```

---

## 🏛️ Clean Architecture Structure

```
backend/
├── docker-compose.yml
├── .env
├── migrations/
│   └── 0001_init.sql                       # PostgreSQL schema (users, rides, indexes)
├── src/
│   ├── domain/                             # 1. Enterprise Business Rules
│   │   ├── entities/ (user.rs, ride.rs)    # Pure Domain Entities
│   │   ├── repositories/                   # Domain Traits / Port Contracts
│   │   │   ├── user_repository.rs
│   │   │   └── ride_repository.rs
│   │   └── errors.rs                       # Typed Domain Errors
│   │
│   ├── application/                        # 2. Application Business Rules
│   │   ├── dto.rs                          # LoginDto, RegisterDto, AuthResponseDto
│   │   ├── services.rs                     # Port traits (PasswordHasher, TokenService)
│   │   └── usecases/
│   │       ├── auth_usecase.rs             # Register, Login, Profile
│   │       └── ride_usecase.rs             # Ride request, Driver match, Ride status
│   │
│   ├── infrastructure/                     # 3. Frameworks & Drivers
│   │   ├── config.rs                       # Environment variables (.env & defaults)
│   │   ├── repositories/
│   │   │   ├── postgres_user_repository.rs # PostgreSQL user persistence (SQLx)
│   │   │   ├── postgres_ride_repository.rs # PostgreSQL ride persistence (SQLx)
│   │   │   ├── in_memory_user_repository.rs
│   │   │   └── in_memory_ride_repository.rs
│   │   └── security/
│   │       ├── password.rs                 # Argon2 password hashing & verification
│   │       └── jwt.rs                      # HS256 JWT generation & validation
│   │
│   ├── presentation/                       # 4. Interface Adapters (Axum HTTP)
│   │   ├── controllers/
│   │   │   ├── auth_controller.rs          # /register, /login, /me
│   │   │   └── ride_controller.rs          # /rides/request, /pending, /:id/match, /status
│   │   ├── middleware.rs                   # Bearer Token Extractor & Error mapping
│   │   ├── state.rs                        # Dependency injection container (AppState)
│   │   └── routes.rs                       # Router, CORS, Health check
│   │
│   ├── lib.rs
│   └── main.rs                             # App startup, DB connection, auto-migrations
└── tests/
    ├── clean_architecture_test.rs          # Isolated unit/usecase tests
    └── postgres_auth_test.rs               # End-to-end PostgreSQL Auth integration tests
```

---

## 🚀 Running the Server

1. **Start PostgreSQL:**
   ```bash
   docker compose up -d
   ```

2. **Run Server:**
   ```bash
   cargo run
   ```
   Migrations in `migrations/0001_init.sql` run automatically on startup. Server listens at `http://localhost:8080`.

3. **Run All Tests:**
   ```bash
   cargo test
   ```

---

## 📡 Authentication API Endpoints

### 1. Register User
`POST /api/v1/auth/register`
```json
{
  "email": "rider@example.com",
  "password": "Password123!",
  "display_name": "Somchai Jaidee",
  "role": "rider"
}
```
**Response (201 Created):**
```json
{
  "token": "eyJhbGciOi...",
  "user": {
    "id": "b3e34bca-...",
    "email": "rider@example.com",
    "display_name": "Somchai Jaidee",
    "photo_url": null,
    "role": "rider",
    "created_at": "2026-10-23T01:15:00Z"
  }
}
```

### 2. Login User
`POST /api/v1/auth/login`
```json
{
  "email": "rider@example.com",
  "password": "Password123!"
}
```
**Response (200 OK):**
```json
{
  "token": "eyJhbGciOi...",
  "user": { ... }
}
```

### 3. Get Current Profile
`GET /api/v1/auth/me`
- **Header:** `Authorization: Bearer <token>`
