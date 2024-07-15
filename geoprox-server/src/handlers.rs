use crate::app::SharedState;
use crate::models::{
    PlaceOrder, PlaceOrderResponse, PlaceRider, PlaceRiderResponse, RemoveRider,
    RemoveRiderResponse,
};
use axum::{extract, Json};

pub async fn place_user(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<PlaceRider>,
) -> Json<PlaceRiderResponse> {
    let mut state = app_state.write().unwrap();
    let precision = state.precision;

    Json(PlaceRiderResponse { 
        geohash: state.position_registry.store_user(&payload.uid, &payload.position, &precision).unwrap() 
    })
}

pub async fn place_contract(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<PlaceOrder>,
) -> Json<PlaceOrderResponse> {
    let mut state = app_state.write().unwrap();
    let precision = state.precision;

    Json(PlaceOrderResponse{
        riders: state.position_registry.add_contract(&payload.position, &payload.distance, &precision).unwrap(),
    })
}

pub async fn remove_user(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<RemoveRider>,
) -> Json<RemoveRiderResponse> {
    let mut state = app_state.write().unwrap();

    Json(RemoveRiderResponse { 
        removed:  state.position_registry.remove_user(&payload.uid),
    })
}
