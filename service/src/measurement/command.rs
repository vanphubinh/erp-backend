use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::measurement::uom::ActiveModel as Uom;
use infra::{util::error, uuid::Uuid};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct CreateUom {
  pub name: String,
}
#[derive(Debug, Serialize)]
pub struct CreateUomMeta {
  pub id: Uuid,
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

pub async fn create_uom_command(
  command: CreateUom,
  db: &impl ConnectionTrait,
) -> Result<CreateUomMeta, DbErr> {
  let uom = Uom {
    name: Set(command.name),
    ..Default::default()
  };
  let uom = uom.insert(db).await?;

  Ok(CreateUomMeta { id: uom.id })
}
