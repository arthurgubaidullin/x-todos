use crate::todo::TodoResource;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Todos {
    service: todos::Service,
}

#[wasm_bindgen]
impl Todos {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            service: todos::Service::new(),
        }
    }

    pub fn all(&self) -> Result<Vec<TodoResource>, JsError> {
        let ab = self
            .service
            .all()
            .map(|list| {
                list.into_iter()
                    .map(TodoResource::from)
                    .collect::<Vec<TodoResource>>()
            })
            .map_err(|e| JsError::new(&e.to_string()))?;

        Ok(ab)
    }

    pub fn add(&self, id: &str, text: &str) -> Result<TodoResource, JsError> {
        let new_todo = todos::NewTodo::new(text);

        self.service
            .add(id, &new_todo)
            .map(TodoResource::from)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

impl Default for Todos {
    fn default() -> Self {
        Self::new()
    }
}
