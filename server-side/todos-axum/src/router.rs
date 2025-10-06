use crate::{
    new_todo_resource::NewTodoResource, todo_list_resource::TodoListResource,
    todo_resource::TodoResource,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};

async fn get_all_todos(State(todos): State<todos::Service>) -> impl IntoResponse {
    todos.all().map(TodoListResource::from).map_err(|error| {
        tracing::error!(error);

        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn add_new_todo(
    Path(todo_id): Path<String>,
    State(todos): State<todos::Service>,
    Json(new_todo): Json<NewTodoResource>,
) -> impl IntoResponse {
    todos
        .add(&todo_id, &todos::NewTodo::new(new_todo.text()))
        .map(TodoResource::from)
        .map(|resource| (StatusCode::CREATED, resource))
        .map_err(|error| {
            tracing::error!(error);

            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub fn router(todos_service: todos::Service) -> Router {
    Router::new()
        .route("/", get(get_all_todos))
        .route("/{id}", put(add_new_todo))
        .with_state(todos_service)
}
