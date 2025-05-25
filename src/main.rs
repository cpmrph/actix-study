use actix_study::{container::Container, create_app::create_app};
use actix_web::HttpServer;
use std::sync::Arc;
use tracing::Level;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let container = Arc::new(Container::new());
    let server =
        HttpServer::new(move || create_app(container.clone())).bind(("127.0.0.1", 8080))?;
    server.run().await
}
