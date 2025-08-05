//! Application Entrypoint
//!
//! This is the main starting point for the backend API using Actix Web + Clean Architecture.
//! Import only what you need, all features are ready to be plugged in.

mod application;
mod business;
mod infrastructure;
mod core;
mod external;

use actix_web::{App, HttpServer, middleware::Logger};
use std::env;
use core::log::init_logger; // assume your logger init in core/log.rs

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ====== ENV & LOGGER INIT ======
    // Example: dotenv::dotenv().ok(); // if you use dotenv
    init_logger(); // Custom logger using tracing/env_logger/log crate

    // Optionally, get port from env
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    println!("🚀 Server running at http://127.0.0.1:{}/", port);

    // ====== HTTP SERVER SETUP ======
    HttpServer::new(move || {
        App::new()
            // --- Global Middleware (Logger, Cors, etc)
            .wrap(Logger::default())
            // --- Register All API Services Here
            .service(application::api::user_controller::get_users)
        // TODO: Add more services from API/Web/CLI, e.g.:
        // .service(application::api::another_controller::get_something)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
