use crate::components::InputSection;

use dioxus::prelude::*;

#[component]
pub fn TodoList() -> Element {
    rsx! {
        div {
            h1 { "Todo List" }
            InputSection {}
        }
    }
}
