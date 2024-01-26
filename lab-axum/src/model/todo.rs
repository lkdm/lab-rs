pub type TodoId = u32;

pub struct Todo {
    pub id: TodoId,
    pub title: String,
    pub completed_at: Option<String>,
    pub archived_at: Option<String>,
}
