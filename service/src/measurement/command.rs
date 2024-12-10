use super::definition::{CreateUomMeta, CreateUomPayload};
use domain::measurement::uom::ActiveModel as Uom;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set};

pub async fn create_uom_command(
  payload: CreateUomPayload,
  db: &impl ConnectionTrait,
) -> Result<CreateUomMeta, DbErr> {
  let uom = Uom {
    name: Set(payload.name),
    ..Default::default()
  };
  let uom = uom.insert(db).await?;

  Ok(CreateUomMeta { id: uom.id })
}
