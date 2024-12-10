use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use infra::util::error;
use sea_orm::DbErr;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum CreateUomError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for CreateUomError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      CreateUomError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (status, error(code, Some("create_uom_command".to_string()))).into_response()
  }
}
