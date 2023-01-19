use crate::ApplicationError;

pub struct Database {}

impl Database {
    pub fn new() -> Result<Database, ApplicationError> {
        Ok(Database {})
    }
}
