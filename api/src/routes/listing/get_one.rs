use axum::{
    http::StatusCode, 
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};


 
pub async fn get_listing_one() -> impl IntoResponse {

    StatusCode::OK.into_response()
}