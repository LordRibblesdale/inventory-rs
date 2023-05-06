pub mod storage;

use log::{debug, info, warn};
use std::io::{BufReader, Read};
use std::path::Path;
use storage::Database;

impl Database {
    pub fn new(database_name: String, database_description: Option<String>) -> Database {
        info!("Default Database created");

        Database {
            name: database_name,
            description: database_description.unwrap_or_default(),
            storage: vec![],
        }
    }

    pub fn load_from_file(file_path: &Path) -> Option<Database> {
        info!("Loading from file {}", file_path.display());

        match std::fs::File::open(file_path) {
            Ok(mut file) => {

                let mut buffer = BufReader::new(file);

                // match Database::decode(buffer) {
                //     Ok(database) => {
                //         info!("Database {} loaded!", database.name);
                //         Some(database)
                //     }
                //     Err(decode_error) => {
                //         warn!(
                //             "Cannot decode database from file {} due to: {}",
                //             file_path.display(),
                //             decode_error
                //         );
                //         None
                //     }
                // }

                None
            }
            Err(file_error) => {
                warn!(
                    "Cannot open file {} due to: {}",
                    file_path.display(),
                    file_error
                );
                None
            }
        }
    }

    pub fn save_to_file(&self, file_path: &Path) -> bool {
        // if let Err(err) = std::fs::write(file_path, self.encode_to_vec()) {
        //     warn!(
        //         "Cannot save database {} to {} due to: {}",
        //         self.name,
        //         file_path.display(),
        //         err
        //     );
        //     return false
        // }

        info!("Database {} saved in {}", self.name, file_path.display());
        true
    }
}

