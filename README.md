# rust-backend-starter

A modular, clean architecture **boilerplate** for building backend RESTful APIs in Rust using [Actix Web](https://actix.rs/).  
This project aims to provide a scalable, easy-to-understand code structure for rapid backend development with Rust, inspired by best practices from Node.js/Express and Go.

---

## ✨ Features

- **Clean folder structure**: Separation of concerns by domain, business logic, and application layer.
- **REST API ready**: Start building endpoints out of the box.
- **Modular codebase**: Easily extend services, controllers, and domain models.
- **Built-in serialization**: Using Serde for JSON request/response.
- **Async runtime**: Powered by Tokio for scalable IO.
- **Ready for extension**: Plug in database, authentication, caching, and more.

---

## 🚀 Getting Started

### 1. **Prerequisites**

- [Rust Toolchain](https://www.rust-lang.org/tools/install) (`rustc`, `cargo`)
- (Optional) [Postman](https://www.postman.com/) or `curl` for testing APIs

### 2. **Structure**

```sh
src/
├── application/            # Application Layer
│   ├── api/                # API Controllers (REST)
│   │   └── mod.rs
│   ├── cli/                # CLI Interface
│   │   └── mod.rs
│   ├── web/                # Web Interface (HTML, templating, etc)
│   │   └── mod.rs
│   └── mod.rs
│
├── business/               # Business Logic Layer
│   ├── domain/             # Domain Models (Entities)
│   │   ├── user.rs
│   │   └── mod.rs
│   ├── services/           # Service Layer (domain service, application service)
│   │   ├── user_service.rs
│   │   └── mod.rs
│   ├── usecase/            # Use Cases (application logic)
│   │   ├── create_user.rs
│   │   └── mod.rs
│   └── mod.rs
│
├── infrastructure/         # Infrastructure Layer
│   ├── concurrency/        # Thread pool, async runtime, channel
│   │   ├── thread_pool.rs
│   │   ├── async_rt.rs
│   │   ├── channel.rs
│   │   └── mod.rs
│   ├── memory/             # In-memory cache, object pool, buffer
│   │   ├── cache.rs
│   │   ├── pool.rs
│   │   ├── buffer.rs
│   │   └── mod.rs
│   ├── io/                 # File/Network IO
│   │   ├── file_io.rs
│   │   ├── net_io.rs
│   │   ├── stream.rs
│   │   └── mod.rs
│   ├── storage/            # Storage Layer (ORM & DB)
│   │   ├── orm/            # Diesel, SQLx, SeaORM
│   │   │   ├── diesel.rs
│   │   │   ├── sqlx.rs
│   │   │   ├── seaorm.rs
│   │   │   └── mod.rs
│   │   ├── query.rs
│   │   ├── migrate.rs
│   │   ├── conn_pool.rs
│   │   ├── blob.rs
│   │   ├── config.rs
│   │   └── mod.rs
│   └── mod.rs
│
├── core/                   # Core Utilities
│   ├── error.rs            # thiserror, anyhow, custom errors
│   ├── log.rs              # tracing, log, env_logger
│   ├── metrics.rs          # metrics/monitoring
│   ├── validation.rs       # input validation
│   └── mod.rs
│
├── external/               # External Systems (API, DB, FS, Network)
│   ├── ext_db.rs
│   ├── ext_api.rs
│   ├── file_sys.rs
│   ├── network.rs
│   └── mod.rs
│
├── main.rs                 # Entrypoint, wiring, server bootstrap
└── lib.rs                  # Library entrypoint (if needed)

