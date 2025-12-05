use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

pub fn run(listener: tokio::net::TcpListener) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { axum::serve(listener, app).await })
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe() -> StatusCode {
    StatusCode::OK
}
