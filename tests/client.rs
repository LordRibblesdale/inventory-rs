//! Client testing module
//! This test shall be executed on one thread to ensure

mod common;

use std::path::Path;
use protobuf::Message;
use uuid::Uuid;
use inventory_rs::storage::database::Database;

#[test]
/// Creates
fn create_empty_database() {
    let db_name = Uuid::new_v4().to_string();
    let database = Database::create_empty_database(db_name.clone(), None);

    assert!(database.is_initialized());
    assert_eq!(database.name, db_name);
}

#[test]
fn save_default_database() {
    let db_name = Uuid::new_v4().to_string();
    let database = Database::create_empty_database(db_name, None);

    let db_path = Path::new("tests").join("bin").join("save").join("database.pb");
    let db_name_path = Path::new("tests").join("bin").join("save").join("name");

    std::fs::write(db_name_path, &database.name).expect("Cannot write database name file");

    assert!(database.save_database_to_file(&db_path));
    assert!(Path::exists(&db_path));
}

#[test]
fn load_default_database() {
    let db_path = Path::new("tests").join("bin").join("load").join("database.pb");
    let db_name_path = Path::new("tests").join("bin").join("load").join("name");
    assert!(Path::exists(&db_path));
    assert!(Path::exists(&db_name_path));

    let database = Database::load_database_from_file(&db_path).expect("Cannot load database file");
    let db_name = std::fs::read_to_string(db_name_path).expect("Cannot load database file name");

    assert_eq!(database.name, db_name);
}


