use std::collections::HashMap;

// Import our Todo struct and TodoId type.
use crate::model::todo::{Todo, TodoId};

// Use once_cell for creating a global variable e.g. our DATA.
use once_cell::sync::Lazy;

// Use Mutex for thread-safe access to a variable e.g. our DATA.
use std::sync::Mutex;

pub struct InMemoryDataStore {
    pub todos: HashMap<TodoId, Todo>,
}

pub static DATA: Lazy<Mutex<InMemoryDataStore>> = Lazy::new(|| {
    Mutex::new(InMemoryDataStore {
        todos: HashMap::from([
            (
                1,
                Todo {
                    id: 1,
                    title: "Learn Rust".to_string(),
                    completed_at: None,
                    archived_at: None,
                },
            ),
            (
                2,
                Todo {
                    id: 2,
                    title: "Learn Axum".to_string(),
                    completed_at: None,
                    archived_at: None,
                },
            ),
            (
                3,
                Todo {
                    id: 3,
                    title: "Learn Tokio".to_string(),
                    completed_at: None,
                    archived_at: None,
                },
            ),
        ]),
    })
});
