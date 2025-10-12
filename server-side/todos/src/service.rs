use crate::{
    NewTodo, in_memory_repository::TodosInMemoryRepository, repository::TodosRepository,
    updated_todo::UpdatedTodo,
};
use todos_rules::Todo;

#[derive(Debug, Clone)]
pub struct Service<Repository>
where
    Repository: TodosRepository,
{
    repository: Repository,
}

impl<Repository> Service<Repository>
where
    Repository: TodosRepository,
{
    #[must_use]
    pub const fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub fn add(
        &self,
        todo_id: &str,
        new_todo: &impl NewTodo,
    ) -> Result<Todo, Box<dyn std::error::Error>> {
        let todo_id = Box::from(todo_id);

        let todo = Todo::new(&todo_id, new_todo.text());

        let todo = self.repository.add(todo)?;

        Ok(todo)
    }

    pub fn get(&self, todo_id: &str) -> Result<Todo, Box<dyn std::error::Error>> {
        self.repository.get(todo_id)
    }

    pub fn update(
        &self,
        todo_id: &str,
        existing_todo: Todo,
        updated_todo: &impl UpdatedTodo,
    ) -> Result<Todo, Box<dyn std::error::Error>> {
        let updating_todo = Todo::new(todo_id, updated_todo.text());

        let updated_todo = self.repository.update(existing_todo, updating_todo)?;

        Ok(updated_todo)
    }

    pub fn remove(&self, todo_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.remove(todo_id)
    }

    pub fn all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        self.repository.all()
    }
}

impl Default for Service<TodosInMemoryRepository> {
    fn default() -> Self {
        Self::new(TodosInMemoryRepository::new())
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
        let todos = Service::new(TodosInMemoryRepository::new());

        assert_eq!(todos.all().unwrap().len(), 0);
    }

    #[test]
    fn add() {
        let new_todo = &NewTodoS::new("Add new todo");

        let todos = Service::new(TodosInMemoryRepository::new());

        todos.add("test", new_todo).unwrap();

        assert_eq!(todos.all().unwrap().len(), 1);
    }

    #[test]
    fn add_twice() {
        let new_todo = &NewTodoS::new("Add new todo");

        let todos = Service::new(TodosInMemoryRepository::new());

        todos.add("test", new_todo).unwrap();

        assert!(todos.add("test", new_todo).is_err());
    }

    #[test]
    fn remove() {
        let new_todo = &NewTodoS::new("Add new todo");

        let todos = Service::new(TodosInMemoryRepository::new());

        let todo_id = "test";
        todos.add(todo_id, new_todo).unwrap();
        todos.remove(todo_id).unwrap();

        assert_eq!(todos.all().unwrap().len(), 0);
    }

    #[test]
    fn remove_not_existed() {
        let todos = Service::new(TodosInMemoryRepository::new());

        let todo_id = "test";

        assert!(todos.remove(todo_id).is_err());
    }
}
