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

## 🗂️ Architecture Overview

Visualisasi arsitektur modular project ini:

```mermaid
graph TB
    %% Application Layer
    subgraph "Application Layer"
        API[API Controllers]
        CLI[CLI Interface]
        WEB[Web Interface]
    end

    %% Business Logic Layer
    subgraph "Business Logic Layer"
        SERVICE[Services]
        DOMAIN[Domain Models]
        USECASE[Use Cases]
    end

    %% Infrastructure Layer
    subgraph "Infrastructure Layer"
        subgraph "Concurrency Management"
            THREAD_POOL[Thread Pool Manager]
            ASYNC_RT[Async Runtime<br/>Tokio/async-std]
            CHANNEL[Channel Communication<br/>mpsc/oneshot]
        end
        
        subgraph "Memory Management"
            CACHE[In-Memory Cache<br/>Arc&lt;RwLock&gt;]
            POOL[Object Pool]
            BUFFER[Buffer Management]
        end
        
        subgraph "I/O Operations"
            FILE_IO[File I/O Handler]
            NET_IO[Network I/O<br/>TCP/UDP/HTTP]
            STREAM[Stream Processing]
        end
        
        subgraph "Storage Layer"
            subgraph "ORM Layer"
                DIESEL[Diesel ORM<br/>Schema & Migrations]
                SQLX[SQLx<br/>Compile-time SQL]
                SEA_ORM[SeaORM<br/>Async ORM]
            end
            
            subgraph "Database Operations"
                QUERY[Query Builder]
                MIGRATE[Migration Manager]
                CONN_POOL[Connection Pool]
            end
            
            BLOB[Blob Storage]
            CONFIG[Configuration Store]
        end
    end

    %% Core Utilities
    subgraph "Core Utilities"
        ERROR[Error Handling<br/>thiserror/anyhow]
        LOG[Logging<br/>tracing/log]
        METRICS[Metrics & Monitoring]
        VALID[Validation]
    end

    %% External Dependencies
    subgraph "External Systems"
        EXT_DB[(External Database)]
        EXT_API[External APIs]
        FILE_SYS[(File System)]
        NETWORK[Network Resources]
    end

    %% Connections - Application to Business
    API --> SERVICE
    CLI --> SERVICE
    WEB --> SERVICE
    
    %% Business Logic Internal
    SERVICE --> USECASE
    USECASE --> DOMAIN
    
    %% Business to Infrastructure
    SERVICE --> THREAD_POOL
    SERVICE --> CACHE
    SERVICE --> DIESEL
    SERVICE --> SQLX
    SERVICE --> SEA_ORM
    
    %% Infrastructure Internal Connections
    THREAD_POOL --> ASYNC_RT
    ASYNC_RT --> CHANNEL
    
    %% Memory connections
    CACHE --> POOL
    POOL --> BUFFER
    
    %% I/O connections
    FILE_IO --> STREAM
    NET_IO --> STREAM
    STREAM --> BUFFER
    
    %% Storage connections
    DIESEL --> QUERY
    SQLX --> QUERY
    SEA_ORM --> QUERY
    QUERY --> CONN_POOL
    MIGRATE --> CONN_POOL
    CONN_POOL --> FILE_IO
    BLOB --> FILE_IO
    CONFIG --> FILE_IO
    
    %% Core Utilities connections
    SERVICE --> ERROR
    ASYNC_RT --> LOG
    THREAD_POOL --> METRICS
    DOMAIN --> VALID
    
    %% External connections
    CONN_POOL --> EXT_DB
    NET_IO --> EXT_API
    FILE_IO --> FILE_SYS
    NET_IO --> NETWORK
    
    %% Styling
    classDef appLayer fill:#e1f5fe
    classDef businessLayer fill:#f3e5f5
    classDef infraLayer fill:#e8f5e8
    classDef coreLayer fill:#fff3e0
    classDef externalLayer fill:#ffebee
    
    class API,CLI,WEB appLayer
    class SERVICE,DOMAIN,USECASE businessLayer
    class THREAD_POOL,ASYNC_RT,CHANNEL,CACHE,POOL,BUFFER,FILE_IO,NET_IO,STREAM,DIESEL,SQLX,SEA_ORM,QUERY,MIGRATE,CONN_POOL,BLOB,CONFIG infraLayer
    class ERROR,LOG,METRICS,VALID coreLayer
    class EXT_DB,EXT_API,FILE_SYS,NETWORK externalLayer


