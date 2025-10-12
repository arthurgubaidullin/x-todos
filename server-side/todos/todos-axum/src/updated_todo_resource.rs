use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatedTodoResource {
    text: Box<str>,
}

impl todos::UpdatedTodo for UpdatedTodoResource {
    fn text(&self) -> &str {
        &self.text
    }
}
