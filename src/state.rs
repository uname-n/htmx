use std::sync::atomic::AtomicUsize;

pub struct AppState {
    pub counter: AtomicUsize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }
}