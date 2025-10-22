use dioxus::prelude::*;

#[derive(Debug, Clone, Props, PartialEq)]
pub struct Todo {
    pub id: u128,
    pub title: String,
    pub completed: bool,
}
