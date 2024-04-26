use rocksdb::{DB, Options};

fn main() {
    // Open the database
    let mut options = Options::default();
    options.create_if_missing(true);
    let db = DB::open_default("/path/to/db").unwrap();

    // Put key-value pair
    let key = b"my_key";
    let value = b"my_value";
    db.put(key, value).unwrap();

    // Get value by key
    let result = db.get(key).unwrap();
    match result {
        Some(value) => {
            println!("Value: {}", String::from_utf8_lossy(&value));
        }
        None => {
            println!("Key not found");
        }
    }
}


