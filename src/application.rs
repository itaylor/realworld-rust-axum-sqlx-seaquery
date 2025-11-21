use tracing::info;
use domain::user_service::UserService;
use http::AppState;
use crate::app_config::load_config;
use crate::database::connect_db;
use crate::{domain, http};
use crate::persistence::user_repository::UserRepository;
use crate::server::init_server;
use crate::tracing::init_tracing;
use crate::utils::hasher::Hasher;
use crate::utils::jwt::JwtHandler;

pub async fn start_app() {
  init_tracing();
  info!("Starting realworld server...");
  let config = load_config();
  
  let app_state = create_app_state(&config).await;

  init_server(&config.http, app_state)
    .await
    .expect("Failed to initialize server");
}

pub async fn create_app_state(
  config: &crate::app_config::AppConfig,
) -> AppState {
  let db = connect_db(&config.database)
    .await
    .expect("Failed to connect to database");

  let jwt_generator = JwtHandler::new(config.secrets.jwt.0.clone());

  let user_service = UserService::new(
    UserRepository::new(db),
    Hasher::new(config.secrets.pepper.0.clone())
  );

  AppState {
    user_service,
    config: config.clone(),
    jwt: jwt_generator,
  }
}
