use rand::seq::SliceRandom;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
pub struct AppState {
  write_db: Arc<DatabaseConnection>,
  read_dbs: Vec<Arc<DatabaseConnection>>,
}

impl AppState {
  pub fn new(write_db: Arc<DatabaseConnection>, read_dbs: Vec<Arc<DatabaseConnection>>) -> Self {
    Self { write_db, read_dbs }
  }

  pub fn get_read_db(&self) -> Arc<DatabaseConnection> {
    let mut rng = rand::thread_rng();
    self.read_dbs.choose(&mut rng).unwrap().clone()
  }

  pub fn get_write_db(&self) -> Arc<DatabaseConnection> {
    self.write_db.clone()
  }
}
