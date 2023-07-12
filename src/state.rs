use std::sync::{atomic::AtomicUsize, Arc, Mutex};

pub struct AppState {
    pub counter: AtomicUsize,
    pub todo_list: Arc<Mutex<Vec<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        let todos = vec![
            "Learn Rust".to_string(),
            "Learn Leptos".to_string(),
            "Learn Actix".to_string(),
        ];
        
        Self {
            counter: AtomicUsize::new(0),
            todo_list: Arc::new(Mutex::new(todos)),
        }
    }
}