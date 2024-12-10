use super::definition::{CreateUomMeta, CreateUomPayload, UpdateUomMeta, UpdateUomPayload};
use domain::measurement::uom::{self, ActiveModel as Uom};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ConnectionTrait, DbErr, Set};

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

pub async fn update_uom_command(
  payload: UpdateUomPayload,
  db: &impl ConnectionTrait,
) -> Result<UpdateUomMeta, DbErr> {
  let uom = uom::ActiveModel {
    id: Set(payload.id),
    name: Set(payload.name),
    created_at: NotSet,
    updated_at: NotSet,
  };
  let updated_uom = uom.update(db).await?;
  Ok(UpdateUomMeta { id: updated_uom.id })
}
