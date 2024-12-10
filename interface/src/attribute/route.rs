use super::handler::list_paginated_attributes;
use axum::{routing::get, Router};
use infra::state::AppState;
use std::sync::Arc;

pub fn new() -> Router<Arc<AppState>> {
  Router::new().route("/attributes.list", get(list_paginated_attributes))
}