use crate::{todo_list_resource::TodoListResource, todo_resource::TodoResource};
use axum::{Router, response::IntoResponse, routing::get};

async fn get_all_todos() -> impl IntoResponse {
    TodoListResource {
        items: vec![TodoResource {
            id: "test".into(),
            text: "lol".into(),
        }],
        next: None,
        previous: None,
    }
}

async fn add_new_todo() -> impl IntoResponse {
    TodoResource {
        id: "test".into(),
        text: "lol".into(),
    }
}

pub fn router() -> Router {
    Router::new().route("/", get(get_all_todos).post(add_new_todo))
}
