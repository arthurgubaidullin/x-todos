use crate::Todo;

pub trait TodosRepository
where
    Self: Send + Sync + Clone + 'static,
{
    fn add(&self, todo: Todo) -> Result<Todo, Box<dyn std::error::Error>>;
    fn get(&self, todo_id: &str) -> Result<Todo, Box<dyn std::error::Error>>;
    fn update(
        &self,
        existing_todo: Todo,
        updated_todo: Todo,
    ) -> Result<Todo, Box<dyn std::error::Error>>;
    fn remove(&self, todo_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>>;
}
