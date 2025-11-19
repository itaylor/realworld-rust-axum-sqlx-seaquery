use crate::app_config::HttpConfig;
use crate::http::{AppState, router};
use axum::Router;
use std::io::Error;
use tracing::info;

pub async fn init_server(config: &HttpConfig, state: AppState) -> Result<(), Error> {
    let listener = tokio::net::TcpListener::bind(config.url()).await?;

    let router = Router::new().nest("/api", router()).with_state(state);

    info!("Starting server on {}", config.url());

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
