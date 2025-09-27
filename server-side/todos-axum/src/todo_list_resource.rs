use crate::todo_resource::TodoResource;
use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoListResource {
    pub(crate) items: Vec<TodoResource>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) previous: Option<Box<str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) next: Option<Box<str>>,
}

impl IntoResponse for TodoListResource {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
