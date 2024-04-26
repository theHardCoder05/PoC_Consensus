use std::collections::{HashMap, VecDeque};
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::oneshot;



#[cfg(test)]
#[path = "tests/store_tests.rs"]
pub mod store_tests;

#[derive(Clone)]
struct Store{
   db: HashMap<String, String>,
    tx: Sender<String>,
}


impl Store {
    fn new(path: &str) -> Result<Store, String> {
        let (tx, mut rx) = channel(100);
        let db = HashMap::new();
        let store = Store { db, tx };

        let mut store_clone = store.clone(); // assuming Store implements Clone

        tokio::spawn(async move {
            while let Some(key) = rx.recv().await {
                store_clone.db.remove(&key);
            }
        });

        Ok(store)
    }

    async fn put(&self, key: String, value: String) -> Result<(), String> {
        self.tx.send(key.clone()).await.map_err(|_| "Failed to send key".to_string())?;
        
        let mut db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            db.insert(key, value);
        }).await.map_err(|_| "Failed to insert into database".to_string())?;

        Ok(())
    }

    async fn get(&self, key: String) -> Result<Option<String>, String>{
        Ok(self.db.get(&key).cloned())
    }
}
