use crate::{components::*, models::Todos};
use dioxus::prelude::*;

#[component]
pub fn TodoList() -> Element {
    let todos = use_hook(Todos::new);

    let mut todos_for_closure = todos.clone();

    let add_todo = move |todo: Todo| {
        todos_for_closure.add_todo(todo);
    };

    rsx! {
        div { class: "bg-dots p-10 min-w-[550px] min-h-[600px] max-h-[700px] overflow-auto rounded-xl",
            h1 { "Todo List" }
            InputSection { add_todo }
            TodosSection { todos: todos.get_todos() }
        }
    }
}
