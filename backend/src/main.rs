use axum::Router;
use tokio::net::TcpListener;

mod error;
mod model;
mod routes;
use model::instanciation::create_db;
use routes::add_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _db = create_db();

    let listener = TcpListener::bind("localhost:8080").await?;

    let mut router: Router = Router::new();

    router = add_routes(router);

    axum::serve(listener, router).await?;
    Ok(())
}
