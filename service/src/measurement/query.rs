use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::measurement::uom::{Column, Entity as Uom, UomDTO};
use infra::{
  response::{PaginatedResponse, PaginationMeta},
  util::error,
  uuid::Uuid,
};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QuerySelect};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPaginatedUomsQuery {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Error, Debug)]
pub enum ListUomsError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListUomsError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListUomsError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("list_paginated_uoms_query".to_string())),
    )
      .into_response()
  }
}

pub async fn list_paginated_uoms_query(
  query: ListPaginatedUomsQuery,
  db: &impl ConnectionTrait,
) -> Result<PaginatedResponse<UomDTO>, DbErr> {
  let per_page = query.per_page.unwrap_or(30);
  let page = query.page.unwrap_or(1) - 1;

  let uom_pages = Uom::find()
    .select_only()
    .column(Column::Id)
    .column(Column::Name)
    .into_partial_model::<UomDTO>()
    .paginate(db, per_page);
  let uoms = uom_pages.fetch_page(page).await?;
  let items_and_pages = uom_pages.num_items_and_pages().await?;
  let total = items_and_pages.number_of_items;
  let total_pages = items_and_pages.number_of_pages;

  Ok(PaginatedResponse::<UomDTO> {
    ok: true,
    data: uoms,
    meta: PaginationMeta {
      page: page + 1,
      total_pages,
      per_page,
      total,
    },
  })
}

#[derive(Debug, Deserialize)]
pub struct FindUomByIdQuery {
  pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct FindUomByIdOutput {
  pub id: Uuid,
  pub name: String,
}

#[derive(Error, Debug)]
pub enum FindUomByIdError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
  #[error("record_not_found")]
  RecordNotFound,
}

impl IntoResponse for FindUomByIdError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      FindUomByIdError::RecordNotFound => (StatusCode::NOT_FOUND, self.to_string()),
      FindUomByIdError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("find_uom_by_id_query".to_string())),
    )
      .into_response()
  }
}

pub async fn find_uom_by_id_query(
  id: Uuid,
  db: &impl ConnectionTrait,
) -> Result<Option<UomDTO>, DbErr> {
  let uom = Uom::find_by_id(id)
    .select_only()
    .column(Column::Id)
    .column(Column::Name)
    .into_partial_model::<UomDTO>()
    .one(db)
    .await?;
  Ok(uom)
}
