use axum::{
    Json,
    http::Uri,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RootResource {
    list: Box<str>,
    create: Box<str>,
}

impl RootResource {
    pub fn new(prefix: &Uri) -> Self {
        let list = format!("{prefix}/items").into();
        let create = format!("{prefix}/items/:id").into();
        Self { list, create }
    }
}

impl IntoResponse for RootResource {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
