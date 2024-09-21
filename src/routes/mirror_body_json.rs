use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct  MirrorJson {
    text: String
}

#[derive(Serialize)]
pub struct  MirrorJsonResponce {
    text: String,
    extra: String
}

pub async fn mirror_body_json (Json(body):Json<MirrorJson>)-> Json<MirrorJsonResponce>{
   Json(MirrorJsonResponce{
        text: body.text ,
        extra: "ohoo..".to_owned(),
    })
}