use axum::{http::StatusCode, response::IntoResponse, Json, response::Response};
use serde_json::json;
use tokio::signal;

pub fn create_response<T>(status: StatusCode, data: Option<T>, message: &str) -> Response
where
    T: serde::Serialize,
{
    let body = json!({
        "data": data,
        "message": message
    });

    (status, Json(body)).into_response()
}


pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}