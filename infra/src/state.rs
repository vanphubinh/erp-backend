use sea_orm::DatabaseConnection;
use std::sync::Arc;
pub struct AppState {
  pub write_db: Arc<DatabaseConnection>,
  pub read_db: Arc<DatabaseConnection>,
}
