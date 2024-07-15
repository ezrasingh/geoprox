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
    Json(PlaceRiderResponse { 
        geohash: state.place_user(payload.uid, payload.position, 10).unwrap() 
    })
}

pub async fn place_order(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<PlaceOrder>,
) -> Json<PlaceOrderResponse> {
    let mut state = app_state.write().unwrap();
    Json(PlaceOrderResponse{
        riders: state.place_contract(payload.position, payload.distance, 10).unwrap(),
    })
}

pub async fn remove_rider(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<RemoveRider>,
) -> Json<RemoveRiderResponse> {
    let mut state = app_state.write().unwrap();
    Json(RemoveRiderResponse { 
        removed:  state.remove_user(&payload.uid),
    })
}
