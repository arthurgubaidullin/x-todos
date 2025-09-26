use crate::todo::Todo;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PersistedTodo {
    id: Arc<str>,
    text: Arc<str>,
}

impl PersistedTodo {
    pub fn new(id: &str, text: &str) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

impl From<&PersistedTodo> for Todo {
    fn from(todo: &PersistedTodo) -> Self {
        Self::new(&todo.id, &todo.text)
    }
}
