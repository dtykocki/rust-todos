use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct TodoList {
    todos: HashMap<usize, Todo>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            todos: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, completed: bool) -> Result<&Todo, String> {
        let next_id = self.todos.len() + 1;
        let todo = Todo::new(next_id, name, completed);
        self.todos.insert(next_id, todo);
        self.find(next_id)
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        match self.todos.remove(&id) {
            Some(_todo) => Ok(()),
            None => Err(String::from("Todo not found")),
        }
    }

    pub fn find(&self, id: usize) -> Result<&Todo, String> {
        match self.todos.get(&id) {
            Some(todo) => Ok(todo),
            None => Err(String::from("Todo not found")),
        }
    }

    pub fn update(&mut self, id: usize, name: &str, completed: bool) -> Result<&Todo, String> {
        match self.todos.get_mut(&id) {
            Some(todo) => {
                todo.name = name.to_string();
                todo.completed = completed;
                Ok(todo)
            }
            None => Err(String::from("Todo not found")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    #[serde(skip_deserializing)]
    id: usize,
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: usize, name: &str, completed: bool) -> Todo {
        Todo {
            id: id,
            name: String::from(name),
            completed: completed,
        }
    }
}
