use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParams{
    hey: String,
    bye:String
}


pub async fn query_params(Query(param):Query<QueryParams>)-> Json<QueryParams>{
    Json(param)
}