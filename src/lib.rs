mod routes;
// use axum::{routing::get, Router};
use routes::create_route;

pub async fn run(){

    let app = create_route();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

