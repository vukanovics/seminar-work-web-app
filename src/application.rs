use crate::database::Database;

#[derive(Debug)]
pub enum ApplicationError {}

pub struct SharedState {
    database: Database,
}

impl SharedState {
    pub fn new() -> Result<SharedState, ApplicationError> {
        Ok(SharedState {
            database: Database::new()?,
        })
    }

    pub fn database(&self) -> &Database {
        return &self.database;
    }
}
