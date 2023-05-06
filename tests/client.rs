use inventory_rs;
use inventory_rs::storage::Database;
use uuid::Uuid;

#[test]
fn create_empty_dataset() {
    let database = Database::new(Uuid::new_v4().to_string(), None);
}
