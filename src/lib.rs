use log::{debug, info};
use std::path::Path;
use protobuf::Message;

pub mod storage;

use storage::database::Database;

impl Database {
    pub fn create_empty_database(database_name: String, database_description: Option<String>) -> Self {
        info!("Default Database created");

        let mut new_database = Self::new();
        new_database.name = database_name;
        new_database.description = database_description.unwrap_or_default();

        new_database
    }

    pub fn load_database_from_file(file_path: &Path) -> Option<Self> {
        info!("Loading from file {}", file_path.display());

        let binary_data = match std::fs::read(file_path) {
            Ok(data) => { data }
            Err(e) => {
                debug!("{}", e);
                return None;
            }
        };

        match Database::parse_from_bytes(&binary_data) {
            Ok(database) => { Some(database) }
            Err(e) => {
                debug!("{}", e);
                None
            }
        }
    }

    pub fn save_database_to_file(&self, file_path: &Path) -> bool {
        let binary_data = match self.write_to_bytes() {
            Ok(data) => { data }
            Err(e) => {
                debug!("{}", e);
                return false;
            }
        };

        match std::fs::write(file_path, binary_data) {
            Ok(_) => { true }
            Err(e) => {
                debug!("{}", e);
                false
            }
        }
    }
}