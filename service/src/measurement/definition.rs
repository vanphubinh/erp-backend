use infra::uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPaginatedUomsQuery {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUomPayload {
  pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUomMeta {
  pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUomPayload {
  pub id: Uuid,
  pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateUomMeta {
  pub id: Uuid,
}
