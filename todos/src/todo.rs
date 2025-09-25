use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Todo {
    text: Arc<str>,
}

impl Todo {
    #[must_use]
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}
