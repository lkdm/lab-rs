use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed_at: Option<String>,
    pub archived_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TodoCreationDto {
    pub title: String,
}
