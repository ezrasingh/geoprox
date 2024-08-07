use axum::{extract::Request, http::header, middleware::Next, response::IntoResponse};

/// tells client to cache this response for some time
pub async fn set_cache_control(req: Request, next: Next) -> impl IntoResponse {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(
        header::CACHE_CONTROL,
        // ? tell client to cache for 3600s (1hr)
        "max-age=3600, immutable".parse().unwrap(),
    );

    response
}

/// tells client the response data is encoded in JSON
pub async fn json_content(req: Request, next: Next) -> impl IntoResponse {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    response
}
