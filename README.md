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

### 2. **Installation**

```sh
# Clone this repository (replace with your repo URL if needed)
git clone https://github.com/yourusername/rust-backend-starter.git
cd rust-backend-starter

# Run the server
cargo run
