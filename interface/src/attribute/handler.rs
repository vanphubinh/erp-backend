use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use infra::state::AppState;
use service::catalog::{
  command::create_attribute_command,
  definition::{CreateAttributePayload, ListPaginatedAttributesQuery},
  error::{CreateAttributeError, ListPaginatedAttributesError},
  query::list_paginated_attributes_query,
};
use std::sync::Arc;

#[axum_macros::debug_handler]
pub async fn list_paginated_attributes(
  State(state): State<Arc<AppState>>,
  Query(params): Query<ListPaginatedAttributesQuery>,
) -> Result<impl IntoResponse, ListPaginatedAttributesError> {
  let attributes = list_paginated_attributes_query(params, &state.read_db).await?;
  Ok((StatusCode::OK, Json(attributes)))
}

#[axum_macros::debug_handler]
pub async fn create_attribute(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateAttributePayload>,
) -> Result<impl IntoResponse, CreateAttributeError> {
  let meta = create_attribute_command(payload, &state.write_db).await?;
  Ok((StatusCode::CREATED, Json(meta)))
}
