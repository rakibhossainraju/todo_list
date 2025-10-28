use dioxus::prelude::*;

use crate::models::Todo;

#[derive(PartialEq, Clone, Props)]
pub struct InputSectionProps {
    pub add_todo: Callback<Todo>,
}

#[component]
pub fn InputSection(props: InputSectionProps) -> Element {
    let mut input_value = use_signal(String::new);

    let handle_submit = move |_| {
        if input_value.read().trim().is_empty() {
            return;
        }

        let todo = Todo {
            id: uuid::Uuid::new_v4().as_u128(),
            title: input_value.read().to_string(),
            completed: false,
        };
        props.add_todo.call(todo);
        input_value.set(String::new());
    };

    let handle_change = move |e: FormEvent| {
        input_value.set(e.data.value());
    };

    rsx! {
        form { onsubmit: handle_submit,
            input {
                class: "outline-none border-b-2 border-white focus:border-blue-500",
                placeholder: "Write Rust code...",
                autocomplete: "off",
                value: "{input_value}",
                oninput: handle_change,
            }
            button { r#type: "submit", "Add Todo" }
        }
    }
}
