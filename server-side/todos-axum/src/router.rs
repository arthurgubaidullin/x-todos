use crate::{todo_list_resource::TodoListResource, todo_resource::TodoResource};
use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};

async fn get_all_todos(State(todos): State<todos::Service>) -> impl IntoResponse {
    todos.all().map(TodoListResource::from).map_err(|error| {
        tracing::error!(error);

        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn add_new_todo() -> impl IntoResponse {
    TodoResource {
        id: "test".into(),
        text: "lol".into(),
    }
}

pub fn router(todos_service: todos::Service) -> Router {
    Router::new()
        .route("/", get(get_all_todos).post(add_new_todo))
        .with_state(todos_service)
}
