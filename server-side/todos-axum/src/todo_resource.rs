use axum::{Json, http::Uri, response::IntoResponse};
use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Serialize)]
struct Links {
    #[serde(rename = "self")]
    itself: Rc<str>,
    remove: Rc<str>,
    update: Rc<str>,
}

#[derive(Debug, Serialize)]
pub struct TodoResource {
    id: Box<str>,
    text: Box<str>,
    links: Links,
}

impl From<(&Uri, todos::Todo)> for TodoResource {
    fn from((uri, value): (&Uri, todos::Todo)) -> Self {
        let link: Rc<str> = format!("{}/items/{}", uri, value.id()).into();
        Self {
            id: value.id().into(),
            text: value.text().into(),
            links: Links {
                itself: link.clone(),
                remove: link.clone(),
                update: link,
            },
        }
    }
}

impl IntoResponse for TodoResource {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
