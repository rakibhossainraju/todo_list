use crate::components::*;
use dioxus::prelude::*;

pub type TodosType = Vec<Todo>;

#[component]
pub fn TodoList() -> Element {
    let mut todos: Signal<TodosType> = use_signal(Vec::new);

    let add_todo = move |todo: Todo| {
        let mut current_todos = todos.read().to_vec();
        current_todos.push(todo);
        todos.set(current_todos);
    };

    rsx! {
        div {
            h1 { "Todo List" }
            InputSection { add_todo }
            TodosSection { todos: todos.read().clone() }
        }
    }
}
