use crate::todo_resource::TodoResource;
use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoList {
    pub(crate) items: Vec<TodoResource>,
}

impl IntoResponse for TodoList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
