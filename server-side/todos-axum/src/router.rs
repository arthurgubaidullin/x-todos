use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

async fn get_all_todos() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn add_new_todo() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

pub fn router() -> Router {
    Router::new().route("/", get(get_all_todos).post(add_new_todo))
}
