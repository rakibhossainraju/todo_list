use crate::components::*;
use dioxus::prelude::*;

pub type TodosType = Signal<Vec<Todo>>;

#[component]
pub fn TodoList() -> Element {
    let mut todos: TodosType = use_signal(Vec::new);

    let add_todo = move |todo: Todo| {
        todos.write().push(todo);
    };

    rsx! {
        div {
            h1 { "Todo List" }
            InputSection { add_todo }
            TodosSection { todos }
        }
    }
}
