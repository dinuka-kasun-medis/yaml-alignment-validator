// Module declarations
mod handlers;
mod models;
mod validator;

// Import necessary crates
use actix_web::{App, HttpServer, web};
use serde::Deserialize;
use std::fs;
use std::io::Result;

/// Struct to hold configuration values
#[derive(Deserialize)]
struct Config {
    ip: String,
    port: u16,
}

/// Main entry point of the application
#[actix_web::main]
async fn main() -> Result<()> {
    let config: Config = load_config();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::index)) 
            .route("/upload", web::post().to(handlers::upload_yaml)) 
    })
    .bind((config.ip.as_str(), config.port))?
    .run()
    .await
}

/// Load and parse the configuration file
fn load_config() -> Config {
    let config_content: String =
        fs::read_to_string("Config.toml").expect("Failed to read configuration file");
    toml::from_str(&config_content).expect("Failed to parse configuration file")
}
