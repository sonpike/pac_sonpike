use std::fs::File;
use std::sync::Arc;

use askama::Template;
use axum::extract::Multipart;
use axum::routing::post;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bytes::Buf;
use gpx::{Track, TrackSegment};
use sqlx::{Pool, Postgres};
use tower_http::services::ServeDir;
use tracing_subscriber::{filter, fmt::layer, prelude::*};

use crate::repositories::get_pool;
use crate::services::handle_gpx;
use crate::templates::HomeTemplate;

mod repositories;
mod services;
mod templates;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    // load .env.local but don't crash yet if it isn't found.
    let _ = dotenvy::from_filename(".env.local").is_ok();
    init_logger();

    let app_state = AppState {
        db: get_pool().await,
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/upload", post(upload))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(page_not_found)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(
        &std::env::var("BIND_ADDR")
            .expect("BIND_ADDR must be set")
            .parse::<String>()
            .unwrap(),
    )
    .await
    .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn init_logger() {
    let file = match File::create("debug.log") {
        Ok(file) => file,
        Err(error) => panic!("Error creating debug file: {:?}", error),
    };

    tracing_subscriber::registry()
        .with(
            layer()
                .pretty()
                .with_filter(filter::LevelFilter::INFO)
                .and_then(layer().with_writer(Arc::new(file))),
        )
        .init();
}

async fn home() -> impl IntoResponse {
    tracing::debug!("New connection request");
    (
        StatusCode::OK,
        Html((HomeTemplate {}).render().unwrap()).into_response(),
    )
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        //let name = field.name().unwrap().to_string();
        let data = field.bytes().await.expect("Could not parse input.");
        handle_gpx(data);
    }
}

async fn page_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Page not found!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
