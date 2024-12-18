use infra::uuid::Uuid;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ListPaginatedCategoriesQuery {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryPayload {
  pub name: String,
  pub parent_category_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CreateCategoryMeta {
  pub id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct UpdateCategoryPayload {
  pub id: Uuid,
  pub name: String,
  pub parent_category_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct UpdateCategoryMeta {
  pub id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ListPaginatedAttributesQuery {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Debug, Serialize, FromQueryResult, Deserialize)]
pub struct AttributeWithOptionsQueryOutput {
  pub id: Uuid,
  pub name: String,
  pub attribute_option_id: Option<Uuid>,
  pub attribute_option_value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct CreateAttributePayload {
  pub name: String,
  pub attribute_options: Vec<CreateAttributeOptionPayload>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAttributeOptionPayload {
  pub value: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAttributeMeta {
  pub id: Uuid,
}
