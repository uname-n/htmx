use std::sync::atomic::AtomicUsize;

pub struct AppState {
    pub counter: AtomicUsize
}