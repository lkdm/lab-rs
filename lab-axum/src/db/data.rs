use std::collections::HashMap;

// Import our Todo struct and TodoId type.
use crate::model::todo::Todo;

// Use once_cell for creating a global variable e.g. our DATA.
use once_cell::sync::Lazy;

// Use Mutex for thread-safe access to a variable e.g. our DATA.
use std::sync::Mutex;

use uuid::Uuid;

pub struct InMemoryDataStore {
    pub todos: HashMap<Uuid, Todo>,
}

pub static DATA: Lazy<Mutex<InMemoryDataStore>> = Lazy::new(|| {
    Mutex::new(InMemoryDataStore {
        todos: HashMap::from([
            (
                uuid::uuid!("00000000-0000-0000-0000-000000000000"),
                Todo {
                    id: uuid::uuid!("00000000-0000-0000-0000-000000000000"),
                    title: "Learn Rust".to_string(),
                    completed_at: None,
                    archived_at: None,
                },
            ),
            (
                uuid::uuid!("00000000-0000-0000-0000-000000000001"),
                Todo {
                    id: uuid::uuid!("00000000-0000-0000-0000-000000000001"),
                    title: "Learn Axum".to_string(),
                    completed_at: None,
                    archived_at: None,
                },
            ),
            (
                uuid::uuid!("00000000-0000-0000-0000-000000000002"),
                Todo {
                    id: uuid::uuid!("00000000-0000-0000-0000-000000000002"),
                    title: "Learn Tokio".to_string(),
                    completed_at: None,
                    archived_at: None,
                },
            ),
        ]),
    })
});
