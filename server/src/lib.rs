use std::sync::Arc;
use libsql::Database;

pub mod database;
pub mod sender;


#[derive(Debug,Clone)]
pub struct AppState{
    pub db: Arc<Database>,
}