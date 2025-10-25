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
        div { class: "bg-dots p-10 min-w-[550px] min-h-[600px] max-h-[700px] overflow-auto rounded-xl",
            h1 { "Todo List" }
            InputSection { add_todo }
            TodosSection { todos }
        }
    }
}
