use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};

use crate::app_state::AppState;
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn get_user(Extension(state): Extension<Arc<AppState>>) -> Json<Value> {
    Json(json!({ "data":  "5"}))
}
