use crate::app_config::load_config;
use crate::database::connect_db;
use crate::domain::user_repository::UserRepository;
use crate::server::init_server;
use crate::tracing::init_tracing;
use ::tracing::info;
use crate::utils::hasher::Hasher;

mod app_config;
mod app_error;
mod database;
mod domain;
mod http;
mod model;
mod server;
mod tracing;
mod utils;

#[tokio::main]
async fn main() {
    init_tracing();
    info!("Starting realworld server...");
    let config = load_config();
    let db = connect_db(&config.database)
        .await
        .expect("Failed to connect to database");

    let user_service = domain::user_service::UserService::new(
        UserRepository::new(db),
        Hasher::new(config.hashing.pepper.0.clone()),
    );

    let app_state = http::AppState {
        user_service,
        config: config.clone(),
    };

    init_server(&config.http, app_state)
        .await
        .expect("Failed to initialize server");
}
