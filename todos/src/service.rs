use crate::{error::Error, new_todo::NewTodo, persisted_todo::PersistedTodo, todo::Todo};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Service {
    db: Arc<RwLock<HashMap<Box<str>, PersistedTodo>>>,
}

impl Service {
    #[must_use]
    pub fn new() -> Self {
        Self { db: Arc::default() }
    }

    pub fn add(
        &self,
        todo_id: &str,
        new_todo: &NewTodo,
    ) -> Result<Todo, Box<dyn std::error::Error>> {
        let todo_id = todo_id.into();

        let persisted_todo = PersistedTodo::new(new_todo.text());

        let todo = Todo::new(new_todo.text());

        self.db
            .write()
            .map_err(|_| Error::Internal)
            .and_then(|mut db| {
                if db.contains_key(&todo_id) {
                    return Err(Error::AlreadyExists);
                }
                db.insert(todo_id, persisted_todo);

                Ok(())
            })?;

        Ok(todo)
    }

    pub fn all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let a = self
            .db
            .read()
            .map_err(|_| Error::Internal)?
            .values()
            .map(Todo::from)
            .collect();

        Ok(a)
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let todos = Service::new();

        assert_eq!(todos.all().unwrap().len(), 0);
    }

    #[test]
    fn add() {
        let new_todo = &NewTodo::new("Add new todo");

        let todos = Service::new();

        todos.add("test", new_todo).unwrap();

        assert_eq!(todos.all().unwrap().len(), 1);
    }

    #[test]
    fn add_twice() {
        let new_todo = &NewTodo::new("Add new todo");

        let todos = Service::new();

        todos.add("test", new_todo).unwrap();

        assert!(todos.add("test", new_todo).is_err());
    }
}
