#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u128,
    pub title: String,
    pub completed: bool,
}
