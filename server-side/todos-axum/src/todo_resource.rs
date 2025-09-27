use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoResource {
    pub(crate) id: Box<str>,
    pub(crate) text: Box<str>,
}

impl IntoResponse for TodoResource {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
