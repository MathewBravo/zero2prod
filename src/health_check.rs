use axum::{http::StatusCode, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn health_check_succeeds() {
        let resp = health_check().await.into_response();

        assert_eq!(resp.status(), StatusCode::OK)
    }
}
