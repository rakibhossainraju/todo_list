use dioxus::prelude::*;

#[component]
pub fn InputSection() -> Element {
    let mut input_value = use_signal(String::new);

    let handle_submit = move |_| {
        input_value.set(String::new());
    };

    let handle_change = move |e: FormEvent| {
        input_value.set(e.data.value());
    };

    rsx! {
        form { onsubmit: handle_submit,
            input {
                placeholder: "Write Rust code...",
                autocomplete: "off",
                value: input_value.read().as_str(),
                oninput: handle_change,
            }
            button { r#type: "submit", "Add Todo" }
        }
    }
}
