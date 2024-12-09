use sea_orm::DatabaseConnection;
pub struct AppState {
  pub write_db: DatabaseConnection,
  pub read_db: DatabaseConnection,
}
