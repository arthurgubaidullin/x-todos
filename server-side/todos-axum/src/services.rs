use axum::http::Uri;

#[derive(Debug, Clone)]
pub struct Services {
    prefix: Uri,
    todos: todos::Service,
}

impl Services {
    pub const fn new(prefix: Uri, todos: todos::Service) -> Self {
        Self { prefix, todos }
    }

    pub const fn prefix(&self) -> &Uri {
        &self.prefix
    }

    pub const fn todos(&self) -> &todos::Service {
        &self.todos
    }
}
