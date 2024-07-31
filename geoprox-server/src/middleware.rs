use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse};

pub async fn set_cache_control(req: Request, next: Next) -> impl IntoResponse {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(
        header::CACHE_CONTROL,
        "max-age=3600, immutable".parse().unwrap(),
    );
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    response
}
