use crate::application::start_app;

mod app_config;
mod app_error;
mod database;
mod domain;
mod http;
mod model;
mod server;
mod tracing;
mod utils;
mod application;
mod persistence;

#[tokio::main]
async fn main() {
  start_app().await
}

