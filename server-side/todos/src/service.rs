use crate::{NewTodo, error::Error, persisted_todo::PersistedTodo, todo::Todo};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
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
        new_todo: &impl NewTodo,
    ) -> Result<Todo, Box<dyn std::error::Error>> {
        let todo_id = Box::from(todo_id);

        let persisted_todo = PersistedTodo::new(&todo_id, new_todo.text());

        let todo = Todo::new(&todo_id, new_todo.text());

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

    pub fn remove(&self, todo_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self
            .db
            .write()
            .map_err(|_| Error::Internal)?
            .remove(todo_id)
            .ok_or(Error::NotExists)
            .map(|_| ())?)
    }

    pub fn all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        Ok(self
            .db
            .read()
            .map_err(|_| Error::Internal)?
            .values()
            .map(Todo::from)
            .collect())
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

    #[derive(Debug, Clone)]
    pub struct NewTodoS {
        text: Box<str>,
    }

    impl NewTodoS {
        #[must_use]
        pub fn new(text: &str) -> Self {
            Self { text: text.into() }
        }
    }

    impl NewTodo for NewTodoS {
        fn text(&self) -> &str {
            &self.text
        }
    }

    #[test]
    fn create() {
        let todos = Service::new();

        assert_eq!(todos.all().unwrap().len(), 0);
    }

    #[test]
    fn add() {
        let new_todo = &NewTodoS::new("Add new todo");

        let todos = Service::new();

        todos.add("test", new_todo).unwrap();

        assert_eq!(todos.all().unwrap().len(), 1);
    }

    #[test]
    fn add_twice() {
        let new_todo = &NewTodoS::new("Add new todo");

        let todos = Service::new();

        todos.add("test", new_todo).unwrap();

        assert!(todos.add("test", new_todo).is_err());
    }

    #[test]
    fn remove() {
        let new_todo = &NewTodoS::new("Add new todo");

        let todos = Service::new();

        let todo_id = "test";
        todos.add(todo_id, new_todo).unwrap();
        todos.remove(todo_id).unwrap();

        assert_eq!(todos.all().unwrap().len(), 0);
    }

    #[test]
    fn remove_not_existed() {
        let todos = Service::new();

        let todo_id = "test";

        assert!(todos.remove(todo_id).is_err());
    }
}
