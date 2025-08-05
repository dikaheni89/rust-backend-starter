# rust-backend-starter

A modular, clean architecture **boilerplate** for building backend RESTful APIs in Rust using [Actix Web](https://actix.rs/).
This project is designed for scalability, readability, and rapid backend development with Rust, inspired by the best patterns in Node.js/Express and Go.

---

## ✨ Features

* **Clean architecture**: Clear separation of Application, Business, Infrastructure, Core, and External layers.
* **REST API ready**: Quickly start building endpoints.
* **Modular codebase**: Easily extend services, controllers, and domain models.
* **Built-in serialization**: Serde for JSON request/response.
* **Async runtime**: Powered by Tokio for scalable IO.
* **Extensible**: Ready for database, authentication, caching, and more.

---

## 🚀 Getting Started

### 1. **Prerequisites**

* [Rust Toolchain](https://www.rust-lang.org/tools/install) (`rustc`, `cargo`)
* (Optional) [Postman](https://www.postman.com/) or `curl` for API testing

---

### 2. **Installation & Usage**

#### **A. Clone the Repository**

```sh
git clone https://github.com/username/rust-backend-starter.git
cd rust-backend-starter
```

#### **B. Build the Project**

```sh
cargo build
```

#### **C. Run the Server**

```sh
cargo run
```

The server will start at:
[http://127.0.0.1:8080/](http://127.0.0.1:8080/)

#### **D. Test the API Endpoint**

For example (users endpoint):

```sh
curl http://127.0.0.1:8080/users
```

---

### 3. **How to Extend or Develop**

* **Add a new API endpoint**:
  Create a new controller in `src/application/api/`, export it in `mod.rs`, and register it in `main.rs`.
* **Add business logic/service**:
  Create a new file in `src/business/services/` or `src/business/usecase/`.
* **Integrate a database**:
  Implement your integration in `src/infrastructure/storage/orm/`.
* **Implement CLI tools**:
  Add your logic in `src/application/cli/`.
* **Validation, logging, metrics**:
  Use and extend utilities in `src/core/`.
* **External system adapters**:
  Integrate external APIs, DBs, or services in `src/external/`.

---

### 4. **Use as a Library in Another Project**

Add to your `Cargo.toml`:

```toml
rust-backend-starter = { git = "https://github.com/dikaheni89/rust-backend-starter.git", branch = "main" }
```

Or as a local dependency:

```toml
rust-backend-starter = { path = "../rust-backend-starter" }
```

Then, simply import any module you need:

```rust
use rust_backend_starter::business::services::user_service::UserService;
use rust_backend_starter::application::api::user_controller::get_users;
```

---

## 🗂️ Folder Structure

```
src/
├── application/            # Application Layer
│   ├── api/                # API Controllers (REST)
│   ├── cli/                # CLI Interface
│   ├── web/                # Web Interface (HTML, templating, etc)
│   └── mod.rs
│
├── business/               # Business Logic Layer
│   ├── domain/             # Domain Models (Entities)
│   ├── services/           # Service Layer
│   ├── usecase/            # Use Cases
│   └── mod.rs
│
├── infrastructure/         # Infrastructure Layer
│   ├── concurrency/        # Thread pool, async runtime, channel
│   ├── memory/             # In-memory cache, object pool, buffer
│   ├── io/                 # File/Network IO
│   ├── storage/            # ORM, query, connection pool, blob, config
│   └── mod.rs
│
├── core/                   # Core Utilities
│   ├── error.rs            # Error handling
│   ├── log.rs              # Logging
│   ├── metrics.rs          # Metrics/monitoring
│   ├── validation.rs       # Input validation
│   └── mod.rs
│
├── external/               # External Systems (API, DB, FS, Network)
│   ├── ext_db.rs
│   ├── ext_api.rs
│   ├── file_sys.rs
│   ├── network.rs
│   └── mod.rs
│
├── main.rs                 # Entrypoint, server bootstrap
└── lib.rs                  # Library entrypoint (optional)
```

---

## 🗂️ Architecture Overview

This diagram visualizes the modular architecture of this project:

<img src="/assets/arsitecture.png" width="100%" height="100%" />

> The diagram above illustrates the relationships between layers (Application, Business Logic, Infrastructure, Core Utilities, and External Systems) according to this folder structure.

---

## 🤝 Contributing

Pull requests and suggestions are welcome!
Feel free to open issues for questions or improvements.

