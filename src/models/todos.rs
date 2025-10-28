use dioxus::{logger::tracing::info, prelude::*};

use crate::{components::Todo, storage::PersistStorage};

pub type TodosType = Signal<Vec<Todo>>;

#[derive(Clone)]
pub struct Todos {
    todos: TodosType,
    storage: PersistStorage,
}

impl Todos {
    const LOCAL_STORAGE_KEY: &'static str = "todos";

    pub fn new() -> Self {
        let storage = PersistStorage::new();
        let todos = Signal::new(Self::load_todos_from_storage(&storage));
        Todos { todos, storage }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.write().push(todo);
        self.add_todo_to_storage();
    }

    pub fn get_todos(&self) -> TodosType {
        self.todos.clone()
    }

    fn add_todo_to_storage(&self) {
        let todos_ref = self.todos.read();
        self.storage.save(Self::LOCAL_STORAGE_KEY, &*todos_ref);
    }

    fn load_todos_from_storage(storage: &PersistStorage) -> Vec<Todo> {
        if let Some(loaded_todos) = storage.load::<Vec<Todo>>(Self::LOCAL_STORAGE_KEY) {
            info!("{:#?}", &loaded_todos);
            return loaded_todos;
        }
        info!("NO Todos fond");
        Vec::new()
    }
}
