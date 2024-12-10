use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use infra::state::AppState;
use service::catalog::{
  definition::ListPaginatedAttributesQuery, error::ListPaginatedAttributesError,
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
