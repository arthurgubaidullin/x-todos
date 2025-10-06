use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct NewTodoResource {
    text: Box<str>,
}

impl NewTodoResource {
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}
