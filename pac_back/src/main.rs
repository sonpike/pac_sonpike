use std::fs::File;
use std::sync::Arc;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing_subscriber::{filter, fmt::layer, prelude::*};

use crate::templates::HomeTemplate;

mod templates;

#[tokio::main]
async fn main() {
    init_logger();

    let app = Router::new()
        .route("/", get(home))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(page_not_found);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
