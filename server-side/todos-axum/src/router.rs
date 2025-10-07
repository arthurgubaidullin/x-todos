use crate::{
    new_todo_resource::NewTodoResource, root_resource::RootResource, services::Services,
    todo_list_resource::TodoListResource, todo_resource::TodoResource,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};

async fn get_all_todos(State(services): State<Services>) -> impl IntoResponse {
    services
        .todos()
        .all()
        .map(|todos| (services.prefix(), todos))
        .map(TodoListResource::from)
        .map_err(|error| {
            tracing::error!(error);

            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn add_new_todo(
    Path(todo_id): Path<String>,
    State(services): State<Services>,
    Json(new_todo): Json<NewTodoResource>,
) -> impl IntoResponse {
    services
        .todos()
        .add(&todo_id, &new_todo)
        .map(|todo| (services.prefix(), todo))
        .map(TodoResource::from)
        .map(|resource| (StatusCode::CREATED, resource))
        .map_err(|error| {
            tracing::error!(error);

            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn resources(State(services): State<Services>) -> impl IntoResponse {
    RootResource::new(services.prefix())
}

pub fn router(
    prefix: &str,
    todos_service: todos::Service,
) -> Result<Router, Box<dyn std::error::Error>> {
    let services = Services::new(prefix.parse()?, todos_service);

    Ok(Router::new()
        .route("/", get(resources))
        .route("/items", get(get_all_todos))
        .route("/items/{id}", put(add_new_todo))
        .with_state(services))
}
