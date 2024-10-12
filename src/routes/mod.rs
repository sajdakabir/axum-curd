mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_custion_hearder;

use axum::{http::Method, routing::{get, post}, Router};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{hard_coded, path_variable};
use query_params::query_params;
use mirror_custion_hearder:: mirror_custion_hearder;
use tower_http::cors::{Any, CorsLayer};

pub fn create_route()-> Router{
    let cors= CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    Router::new().route("/hello", get(hello_world))
                .route("/mirror_body_string", post(mirror_body_string))
                .route("/mirror_body_json", post(mirror_body_json))
                .route("/path_variable/:id", get(path_variable))
                .route("/path_variable/100", get(hard_coded))
                .route("/queary", get(query_params))
                .route("/header", get(mirror_custion_hearder))
                .layer(cors)
}