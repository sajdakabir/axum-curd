use axum::http::HeaderMap;

pub async fn mirror_custion_hearder(headers: HeaderMap)-> String{
    let message_value = headers.get("x-get-msg").unwrap();
    let message = message_value.to_str().unwrap().to_owned();
    message
}