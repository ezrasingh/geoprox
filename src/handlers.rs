use crate::app::SharedState;
use crate::models::{
    PlaceOrder, PlaceOrderResponse, PlaceRider, PlaceRiderResponse, RemoveRider,
    RemoveRiderResponse,
};
use axum::{extract, Json};

pub async fn place_rider(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<PlaceRider>,
) -> Json<PlaceRiderResponse> {
    let mut state = app_state.write().unwrap();
    let res = state.place_rider(payload);
    Json(res)
}

pub async fn place_order(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<PlaceOrder>,
) -> Json<PlaceOrderResponse> {
    let state = app_state.read().unwrap();
    let res = state.clone().place_order(payload);
    Json(res)
}

pub async fn remove_rider(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<RemoveRider>,
) -> Json<RemoveRiderResponse> {
    let mut state = app_state.write().unwrap();
    let res = state.remove_rider(payload);
    Json(res)
}
