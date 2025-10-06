use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct NewTodoResource {
    text: Box<str>,
}

impl todos::NewTodo for NewTodoResource {
    fn text(&self) -> &str {
        &self.text
    }
}
