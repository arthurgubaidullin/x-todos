use sha2::{Digest, Sha256};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Todo {
    id: Arc<str>,
    text: Arc<str>,
}

impl Todo {
    #[must_use]
    pub fn new(id: &str, text: &str) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn update(&mut self, text: &str) {
        self.text = text.into();
    }

    #[must_use]
    pub fn version(&self) -> Box<str> {
        let mut hasher = Sha256::new();
        hasher.update(self.text());
        let hash = hasher.finalize();
        format!("{hash:x}").into()
    }
}
