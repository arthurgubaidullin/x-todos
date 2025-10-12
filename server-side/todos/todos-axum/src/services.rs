use axum::http::Uri;
use todos::TodosRepository;

#[derive(Debug, Clone)]
pub struct Services<Repository>
where
    Repository: TodosRepository,
{
    prefix: Uri,
    todos: todos::Service<Repository>,
}

impl<Repository> Services<Repository>
where
    Repository: TodosRepository,
{
    pub const fn new(prefix: Uri, todos: todos::Service<Repository>) -> Self {
        Self { prefix, todos }
    }

    pub const fn prefix(&self) -> &Uri {
        &self.prefix
    }

    pub const fn todos(&self) -> &todos::Service<Repository> {
        &self.todos
    }
}
