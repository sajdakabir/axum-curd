mod hello_world;
use axum::{routing::{get, post}, Router};

use hello_world::hello_world;

pub fn create_route()-> Router{
    Router::new().route("/hello", get(hello_world))
    // Router::new().route("/hello", post(hello_world))
}

