use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct TodoResource {
    id: String,
    text: String,
}

#[wasm_bindgen]
impl TodoResource {
    #[must_use]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[must_use]
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

impl From<todos::Todo> for TodoResource {
    fn from(value: todos::Todo) -> Self {
        Self {
            id: value.id().into(),
            text: value.text().into(),
        }
    }
}
