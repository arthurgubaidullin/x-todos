use std::{
    collections::HashMap,
    ops::Not,
    sync::{Arc, RwLock},
};

use crate::{Error, Todo, persisted_todo::PersistedTodo, repository::TodosRepository};

#[derive(Debug, Clone)]
pub struct TodosInMemoryRepository {
    db: Arc<RwLock<HashMap<Box<str>, PersistedTodo>>>,
}

impl TodosInMemoryRepository {
    #[must_use]
    pub fn new() -> Self {
        Self { db: Arc::default() }
    }
}

impl Default for TodosInMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TodosRepository for TodosInMemoryRepository {
    fn add(&self, todo: Todo) -> Result<Todo, Box<dyn std::error::Error>> {
        let persisted_todo = PersistedTodo::from(&todo);

        self.db
            .write()
            .map_err(|_| Error::Internal)
            .and_then(|mut db| {
                if db.contains_key(persisted_todo.id()) {
                    return Err(Error::AlreadyExists);
                }

                db.insert(persisted_todo.id().into(), persisted_todo);

                Ok(())
            })?;

        Ok(todo)
    }

    fn get(&self, todo_id: &str) -> Result<Todo, Box<dyn std::error::Error>> {
        Ok(self
            .db
            .read()
            .map_err(|_| Error::Internal)?
            .get(todo_id)
            .ok_or(Error::NotExists)
            .map(Todo::from)?)
    }

    fn update(
        &self,
        existing_todo: Todo,
        updated_todo: Todo,
    ) -> Result<Todo, Box<dyn std::error::Error>> {
        Ok(self
            .db
            .write()
            .map_err(|_| Error::Internal)
            .and_then(|mut db| {
                let existing_todo_in_db = db
                    .get(existing_todo.id())
                    .ok_or(Error::NotExists)
                    .map(Todo::from)?;

                if existing_todo_in_db
                    .version()
                    .eq(&existing_todo.version())
                    .not()
                {
                    return Err(Error::Conflict);
                }

                let persisted_todo = PersistedTodo::from(&updated_todo);

                let _ = db.insert(persisted_todo.id().into(), persisted_todo);

                Ok(updated_todo)
            })?)
    }

    fn remove(&self, todo_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self
            .db
            .write()
            .map_err(|_| Error::Internal)?
            .remove(todo_id)
            .ok_or(Error::NotExists)
            .map(|_| ())?)
    }

    fn all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        Ok(self
            .db
            .read()
            .map_err(|_| Error::Internal)?
            .values()
            .map(Todo::from)
            .collect())
    }
}
