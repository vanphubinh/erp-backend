use async_trait::async_trait;
use chrono::Utc;
use infra::uuid::Uuid;
use sea_orm::{entity::prelude::*, ActiveModelTrait, DerivePartialModel, FromQueryResult, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "uom")]
#[serde(rename_all = "camelCase")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::new()),
      ..ActiveModelTrait::default()
    }
  }

  async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    let _ = db;
    let mut this = self;
    if insert {
      let now = Utc::now().into();
      this.created_at = Set(now);
      this.updated_at = Set(now);
    } else {
      this.updated_at = Set(Utc::now().into());
    }
    Ok(this)
  }
}

#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult)]
#[sea_orm(entity = "Entity")]
#[serde(rename_all = "camelCase")]
pub struct UomDTO {
  pub id: Uuid,
  pub name: String,
}
