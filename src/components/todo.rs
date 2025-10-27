use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u128,
    pub title: String,
    pub completed: bool,
}
