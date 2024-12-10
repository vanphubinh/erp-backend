use axum::{
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use infra::state::AppState;
use service::catalog::{
  definition::ListPaginatedCategoriesQuery, error::ListCategoriesError,
  query::list_paginated_categories_query,
};
use std::sync::Arc;

#[axum_macros::debug_handler]
pub async fn list_paginated_categories(
  State(state): State<Arc<AppState>>,
  Query(params): Query<ListPaginatedCategoriesQuery>,
) -> Result<impl IntoResponse, ListCategoriesError> {
  let categories = list_paginated_categories_query(params, &state.read_db).await?;
  Ok((StatusCode::OK, Json(categories)))
}
