use axum::Router;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
pub async fn start() {
  dotenvy::dotenv().ok();

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let tcp = TcpListener::bind(&addr).await.unwrap();

  let router = Router::new().layer(
    TraceLayer::new_for_http().make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO)),
  );

  tracing::debug!("Listening on http://{}", addr);
  axum::serve(tcp, router).await.unwrap();
}

pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
  let db_url = std::env::var("DATABASE_URL").unwrap();
  let db = Database::connect(&db_url).await?;
  Ok(db)
}
