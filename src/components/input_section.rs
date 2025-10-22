use dioxus::prelude::*;

#[component]
pub fn InputSection() -> Element {
    let mut input_value = use_signal(String::new);

    let handle_submit = move |_| {
        if input_value.read().trim().is_empty() {
            return;
        }
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
                value: input_value.read().as_str(),
                oninput: handle_change,
            }
            button { r#type: "submit", "Add Todo" }
        }
    }
}
