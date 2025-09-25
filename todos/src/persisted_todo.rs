use crate::todo::Todo;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PersistedTodo {
    text: Arc<str>,
}

impl PersistedTodo {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }
}

impl From<&PersistedTodo> for Todo {
    fn from(todo: &PersistedTodo) -> Self {
        Self::new(&todo.text)
    }
}
