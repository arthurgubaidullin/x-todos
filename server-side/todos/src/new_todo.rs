#[derive(Debug, Clone)]
pub struct NewTodo {
    text: Box<str>,
}

impl NewTodo {
    #[must_use]
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}
