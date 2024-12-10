//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use async_trait::async_trait;
use infra::uuid::Uuid;
use sea_orm::{entity::prelude::*, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

use super::attribute_option::AttributeOptionDTO;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "attribute")]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::attribute_option::Entity")]
  AttributeOptions,
}

impl Related<super::attribute_option::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::AttributeOptions.def()
  }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::new()),
      ..ActiveModelTrait::default()
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeWithOptions {
  pub id: Uuid,
  pub name: String,
  pub attribute_options: Vec<AttributeOptionDTO>,
}
