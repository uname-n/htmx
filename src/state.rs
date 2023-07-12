use std::sync::Arc;
use libsql_client::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
}