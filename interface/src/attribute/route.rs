use super::handler::{create_attribute, list_paginated_attributes};
use axum::{
  routing::{get, post},
  Router,
};
use infra::state::AppState;
use std::sync::Arc;

pub fn new() -> Router<Arc<AppState>> {
  Router::new()
    .route("/attributes.list", get(list_paginated_attributes))
    .route("/attributes.create", post(create_attribute))
}
