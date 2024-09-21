use axum::extract::Path;



pub async fn path_variable(Path(id):Path<i32>)->String{
    id.to_string()
}
pub async fn hard_coded()->String{
   "this is me".to_owned()
}