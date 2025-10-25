use dioxus::prelude::*;

use crate::todo_list::TodosType;

#[derive(PartialEq, Clone, Props)]
pub struct TodosSectionProps {
    pub todos: TodosType,
}

#[component]
pub fn TodosSection(props: TodosSectionProps) -> Element {
    rsx! {
        div { class: "todos-section",
            h2 { "Todos" }
            ul {
                {props.todos.iter().map(|todo| rsx! {
                    li { key: todo.id, {todo.title.as_str()} }
                })}
            }
        }
    }
}
