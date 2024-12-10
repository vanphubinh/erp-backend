use infra::uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
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
pub struct UpdateCategoryPayload {
  pub id: Uuid,
  pub name: String,
  pub parent_category_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct UpdateCategoryMeta {
  pub id: Uuid,
}
