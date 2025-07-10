# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository contains example code for "LUVIT 실전 백엔드 러스트 Axum 프로그래밍" (Practical Backend Rust Axum Programming), a Korean book about building web applications with Rust and Axum. The codebase includes multiple independent projects demonstrating different aspects of Axum web development.

## Architecture

The repository is organized into five main projects:

1. **axum-react-chat/**: Full-stack chat application with React frontend and Axum backend
   - `frontend/`: React application using Vite, Chakra UI, and WebSocket/SSE
   - `backend/`: Axum server with SeaORM, PostgreSQL, and Shuttle deployment

2. **axum-rest-seaorm/**: RESTful API server with authentication and database operations
   - Uses SeaORM with PostgreSQL
   - JWT authentication with bcrypt hashing
   - Product and category management endpoints
   - Swagger UI documentation

3. **axum-rest-basic/**: Basic REST API server example
   - Simple API server implementation

4. **axum-proxy-server/**: HTTP proxy server demonstrating request forwarding
   - Simple proxy implementation using reqwest

5. **axum-websocket/**: WebSocket server example
   - Basic WebSocket echo server implementation

## Development Commands

### Chat Application (axum-react-chat)

Frontend development:
```bash
cd axum-react-chat/frontend
yarn install
yarn dev          # Start Vite dev server (default port 5173)
yarn build        # Build for production
yarn lint         # Run ESLint
yarn preview      # Preview production build
```

Backend development:
```bash
cd axum-react-chat/backend

# For Shuttle deployment
cargo run --features shuttle         # Run with Shuttle (requires shuttle login)
shuttle run --port 3000              # Alternative Shuttle command
shuttle deploy --features shuttle    # Deploy to Shuttle

# For Docker/local development
cargo run --bin docker               # Run Docker version (port 3000)
cargo build --bin docker             # Build Docker binary

# Using Docker Compose
docker-compose up --build            # Build and run with PostgreSQL
docker-compose up --build -d         # Run in background
docker-compose down                   # Stop and remove containers
```

### REST API with SeaORM (axum-rest-seaorm)

```bash
cd axum-rest-seaorm
cargo run         # Start the API server (port 8000)
cargo test        # Run tests
```

Access Swagger UI at: http://localhost:8000/swagger-ui/

### Other Projects

```bash
cd axum-rest-basic
cargo run         # Start basic REST API server

cd axum-proxy-server
cargo run         # Start proxy server

cd axum-websocket
cargo run         # Start WebSocket server
```

## Database Setup

Both the chat app and REST API use PostgreSQL with SeaORM:

- Migration files are in `migration/` directories
- Database initialization is handled in `src/db/init.rs`
- Entity definitions are in `src/entities/`

### Chat App Database
- **Shuttle**: Database provisioning is handled automatically
- **Docker**: PostgreSQL runs in container with credentials:
  - Host: localhost:5432
  - Database: axum_react_chat
  - User: axum
  - Password: 1234

### REST API Database
For local development, set up PostgreSQL and configure connection via environment variables (.env file).

## Key Technologies

- **Axum**: Web framework with extractors, handlers, and middleware
- **SeaORM**: Database ORM with migration support
- **tokio**: Async runtime
- **tower-http**: HTTP middleware (compression, tracing, CORS)
- **React**: Frontend framework (chat app only)
- **Vite**: Frontend build tool
- **Shuttle**: Deployment platform (chat app backend)
- **Chakra UI**: React component library (chat app frontend)
- **JWT**: Authentication tokens (REST API)
- **bcrypt**: Password hashing (REST API)
- **utoipa**: OpenAPI/Swagger documentation (REST API)

## Project Architecture

### Chat Application (`axum-react-chat`)
- **Frontend**: React SPA with Vite build system
- **Backend**: Axum server with conditional compilation for Shuttle vs Docker
- **Database**: PostgreSQL with SeaORM
- **Features**: Real-time chat with Server-Sent Events (SSE)
- **Deployment**: Shuttle for production, Docker for local development

### REST API (`axum-rest-seaorm`)
- **Architecture**: Layered architecture with API, entities, and utilities
- **Authentication**: JWT middleware for protected routes
- **Documentation**: Auto-generated Swagger UI
- **Database**: PostgreSQL with SeaORM migrations
- **Features**: User management, product/category CRUD, text processing

## Testing

The REST API includes test setup. Tests can be run with `cargo test` in the respective directories.