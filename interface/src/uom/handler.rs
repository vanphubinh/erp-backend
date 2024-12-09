use axum::{
  extract::{Path, Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use infra::{state::AppState, uuid::Uuid};
use service::measurement::{
  command::{create_uom_command, CreateUom, CreateUomError},
  query::{
    find_uom_by_id_query, list_paginated_uoms_query, FindUomByIdError, ListPaginatedUomsQuery,
    ListUomsError,
  },
};
use std::sync::Arc;

#[axum_macros::debug_handler]
pub async fn list_paginated_uoms(
  State(state): State<Arc<AppState>>,
  Query(params): Query<ListPaginatedUomsQuery>,
) -> Result<impl IntoResponse, ListUomsError> {
  let uoms = list_paginated_uoms_query(params, &state.read_db).await?;
  Ok((StatusCode::OK, Json(uoms)))
}

#[axum_macros::debug_handler]
pub async fn create_uom(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateUom>,
) -> Result<impl IntoResponse, CreateUomError> {
  let meta = create_uom_command(payload, &state.write_db).await?;
  Ok((StatusCode::OK, Json(meta)))
}

#[axum_macros::debug_handler]
pub async fn find_uom_by_id(
  State(state): State<Arc<AppState>>,
  Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, FindUomByIdError> {
  let uom = match find_uom_by_id_query(id, &state.read_db).await {
    Ok(Some(uom)) => uom,
    Ok(None) => return Err(FindUomByIdError::RecordNotFound),
    Err(e) => return Err(FindUomByIdError::InternalServerError(e)),
  };
  Ok((StatusCode::OK, Json(uom)))
}
