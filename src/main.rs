use std::sync::Arc;

use aws_sdk_s3::{Client, config::Credentials};
use aws_config::{Region, meta::region::RegionProviderChain};
use axum::{handler::Handler, http::{HeaderValue, Method, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}}};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

use crate::{config::Config, database::DBClient, routes::create_router};

pub mod config;
pub mod database;
pub mod models;
pub mod dtos;
pub mod error;
pub mod middleware;
pub mod utils;
pub mod handler;
pub mod routes;

#[derive(Debug,Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client:DBClient,
    pub s3_client: Client,

}

#[tokio::main]
async fn  main() {
tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();

    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.db_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::OPTIONS])
        .allow_credentials(true);

    let db_client = DBClient::new(pool);

    let creds = Credentials::new(
    config.aws_access_key.clone(),
    config.aws_secret_key.clone(),
    None,
    None,
    "env",
);

    let region_provider = RegionProviderChain::first_try(
         Region::new(config.aws_region.clone())
    );

    let shared_config = aws_config::from_env()
        .region(region_provider)
        .credentials_provider(creds)
        .load()
        .await;

let s3_client = aws_sdk_s3::Client::new(&shared_config);

    let app_state = Arc::new(AppState {
        db_client,
        env: config.clone(),
        s3_client 
    });

    let app = create_router(app_state).layer(cors);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    println!("Server running on port {}", config.port);

    axum::serve(listener, app).await.unwrap();
}