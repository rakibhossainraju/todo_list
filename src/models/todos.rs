use dioxus::{logger::tracing::info, prelude::*};

use crate::components::Todo;

pub type TodosType = Signal<Vec<Todo>>;

#[derive(Clone, PartialEq)]
pub struct Todos {
    todos: TodosType,
}

impl Todos {
    const LOCAL_STORAGE_KEY: &'static str = "todos";

    pub fn new() -> Self {
        info!("Creating new Todos instance");
        let todos = Signal::new(Self::load_todos_from_storage());
        Todos { todos }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.write().push(todo);
        self.add_todo_to_storage();
    }

    pub fn get_todos(&self) -> TodosType {
        self.todos.clone()
    }

    fn add_todo_to_storage(&self) {
        #[cfg(feature = "web")]
        {
            let todos_ref = self.todos.read();
            storage_web::save(Self::LOCAL_STORAGE_KEY, &*todos_ref);
        }
    }
    fn load_todos_from_storage() -> Vec<Todo> {
        #[cfg(feature = "web")]
        {
            if let Some(loaded_todos) = storage_web::load::<Vec<Todo>>(Self::LOCAL_STORAGE_KEY) {
                return loaded_todos;
            }
        }
        Vec::new()
    }
}

// #[cfg(feature = "web")]
mod storage_web {
    use serde_json;
    use web_sys::{window, Storage};

    fn get_local_storage() -> Storage {
        window()
            .expect("Could not get the window")
            .local_storage()
            .expect("Could not get local storage")
            .expect("Local storage is not available")
    }

    pub fn save<T: serde::Serialize>(key: &str, value: &T) {
        let storage = get_local_storage();
        let value_str = serde_json::to_string(value).expect("Could not serialize value");
        storage.set_item(key, &value_str).expect("Could not set item in local storage");
    }

    pub fn load<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
        let storage = get_local_storage();
        let value = storage.get_item(key).ok()??;
        serde_json::from_str(&value).ok()
    }
}
