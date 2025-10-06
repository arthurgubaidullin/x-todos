use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoResource {
    pub(crate) id: Box<str>,
    pub(crate) text: Box<str>,
}

impl From<todos::Todo> for TodoResource {
    fn from(value: todos::Todo) -> Self {
        Self {
            id: value.id().into(),
            text: value.text().into(),
        }
    }
}

impl IntoResponse for TodoResource {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
