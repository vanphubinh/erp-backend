use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use infra::util::error;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ListCategoriesError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListCategoriesError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListCategoriesError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("list_paginated_categories_query".to_string())),
    )
      .into_response()
  }
}

#[derive(Error, Debug)]
pub enum FindCategoryByIdError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
  #[error("record_not_found")]
  RecordNotFound,
}

impl IntoResponse for FindCategoryByIdError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      FindCategoryByIdError::RecordNotFound => (StatusCode::NOT_FOUND, self.to_string()),
      FindCategoryByIdError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("find_category_by_id_query".to_string())),
    )
      .into_response()
  }
}

#[derive(Error, Debug)]
pub enum CreateCategoryError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for CreateCategoryError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      CreateCategoryError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("create_category_command".to_string())),
    )
      .into_response()
  }
}

#[derive(Error, Debug)]
pub enum UpdateCategoryError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for UpdateCategoryError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      UpdateCategoryError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("update_category_command".to_string())),
    )
      .into_response()
  }
}

#[derive(Error, Debug)]
pub enum ListPaginatedAttributesError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListPaginatedAttributesError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListPaginatedAttributesError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("list_paginated_attributes_query".to_string())),
    )
      .into_response()
  }
}
