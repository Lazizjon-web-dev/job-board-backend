# Job Board Backend

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Actix](https://img.shields.io/badge/Actix--web-000000?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

A RESTful API backend for a job board application built with Rust, Actix-web, and PostgreSQL. This application provides endpoints for user authentication, job listings management, and job application handling.

## Features

- **User Management**
  - User registration and authentication with JWT
  - Role-based access control (`job_seeker`, `employer`)
  - Secure password hashing with bcrypt

- **Job Listings**
  - Full CRUD operations for job postings
  - Employers can create, update, and delete their own job listings
  - Job seekers can browse all available jobs

- **Applications**
  - Job seekers can apply to jobs with a message
  - Application status tracking (`pending`, `accepted`, `rejected`)
  - Full CRUD operations for applications

## Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **Web Framework**: [Actix-web](https://actix.rs/)
- **Database**: [PostgreSQL](https://www.postgresql.org/)
- **ORM/Query Builder**: [SQLx](https://github.com/launchbadge/sqlx)
- **Authentication**: JWT ([jsonwebtoken](https://crates.io/crates/jsonwebtoken))
- **Password Hashing**: [bcrypt](https://crates.io/crates/bcrypt)
- **Serialization**: [Serde](https://serde.rs/)

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/download/) (v12 or higher)
- [sqlx-cli](https://crates.io/crates/sqlx-cli) for database migrations

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres
```

## Installation & Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/Lazizjon-web-dev/job-board-backend.git
   cd job-board-backend
   ```

2. **Configure environment variables**

   Create a `.env` file in the project root:

   ```bash
   DATABASE_URL=postgres://username:password@localhost/job_board
   JWT_SECRET=your_secret_key_here
   ```

   Replace `username`, `password`, and `job_board` with your PostgreSQL credentials and desired database name.

3. **Create the database**

   ```bash
   createdb job_board
   ```

4. **Run database migrations**

   ```bash
   sqlx migrate run
   ```

5. **Build and run the server**

   ```bash
   cargo run
   ```

   The server will start on `http://localhost:8000`.

## API Documentation

### Authentication

| Method | Endpoint             | Description         | Auth Required |
|--------|----------------------|---------------------|---------------|
| POST   | `/api/auth/register` | Register a new user | No            |
| POST   | `/api/auth/login`    | Login user          | No            |

### Users

| Method | Endpoint                    | Description                      | Auth Required |
|--------|-----------------------------|----------------------------------|---------------|
| GET    | `/api/users/{id}`           | Get user by ID                   | No            |
| GET    | `/api/users/{id}/jobs`      | Get all jobs posted by a user    | No            |
| GET    | `/api/users/{id}/applications` | Get all applications by a user | No            |

### Jobs

| Method | Endpoint          | Description          | Auth Required |
|--------|-------------------|----------------------|---------------|
| GET    | `/api/jobs`       | Get all jobs         | No            |
| GET    | `/api/jobs/{id}`  | Get job by ID        | No            |
| POST   | `/api/jobs`       | Create a new job     | Yes           |
| PUT    | `/api/jobs/{id}`  | Update a job         | Yes           |
| DELETE | `/api/jobs/{id}`  | Delete a job         | Yes           |

### Applications

| Method | Endpoint                | Description              | Auth Required |
|--------|-------------------------|--------------------------|---------------|
| GET    | `/api/applications`     | Get all applications     | No            |
| GET    | `/api/applications/{id}`| Get application by ID    | No            |
| POST   | `/api/applications`     | Create a new application | Yes           |
| PUT    | `/api/applications/{id}`| Update an application    | Yes           |
| DELETE | `/api/applications/{id}`| Delete an application    | Yes           |

**Note**: For authenticated endpoints, include the JWT token in the `Authorization` header.

## Database Schema

### Users Table

| Column        | Type      | Description                               |
|---------------|-----------|-------------------------------------------|
| id            | SERIAL    | Primary key                               |
| username      | TEXT      | Unique username                           |
| email         | TEXT      | Unique email address                      |
| password_hash | TEXT      | Bcrypt hashed password                    |
| role          | TEXT      | User role (`job_seeker` or `employer`)    |
| created_at    | TIMESTAMP | Record creation timestamp                 |
| updated_at    | TIMESTAMP | Record last update timestamp              |

### Jobs Table

| Column      | Type         | Description                        |
|-------------|--------------|------------------------------------|
| id          | SERIAL       | Primary key                        |
| title       | TEXT         | Job title                          |
| description | TEXT         | Job description                    |
| location    | TEXT         | Job location                       |
| salary      | NUMERIC(10,2)| Job salary                         |
| category    | TEXT         | Job category                       |
| employer_id | INTEGER      | Foreign key to users table         |
| created_at  | TIMESTAMP    | Record creation timestamp          |
| updated_at  | TIMESTAMP    | Record last update timestamp       |

### Applications Table

| Column     | Type      | Description                                           |
|------------|-----------|-------------------------------------------------------|
| id         | SERIAL    | Primary key                                           |
| job_id     | INTEGER   | Foreign key to jobs table                             |
| user_id    | INTEGER   | Foreign key to users table                            |
| message    | TEXT      | Application message                                   |
| status     | TEXT      | Application status (`pending`, `accepted`, `rejected`)|
| created_at | TIMESTAMP | Record creation timestamp                             |
| updated_at | TIMESTAMP | Record last update timestamp                          |

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature-name`)
3. Make your changes
4. Run tests and ensure the code compiles (`cargo build`)
5. Commit your changes (`git commit -m 'Add some feature'`)
6. Push to the branch (`git push origin feature/your-feature-name`)
7. Open a Pull Request

Please ensure your code follows the existing style and includes appropriate documentation.

## License

This project is open source.
