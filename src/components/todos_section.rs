use dioxus::prelude::*;

#[component]
pub fn TodosSection() -> Element {
    let todos = vec!["Todo 1", "Todo 2", "Todo 3"];
    rsx! {
        div {
            h2 { "Todos" }
            ul {
                {todos.iter().map(|todo| rsx! {
                    li { "{todo}" }
                })}
            }
        }
    }
}
