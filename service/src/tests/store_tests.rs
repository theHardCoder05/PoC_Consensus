use super::*;
use std::fs;

use tokio::sync::mpsc::channel;
use std::collections::HashMap;
    #[tokio::test]
    async fn create_store() {
        // Create new store.
        let path = ".db_test_create_store";
        let _ = fs::remove_dir_all(path);
        let store = Store::new(path);
        assert!(store.is_ok());
    }
    
    
    #[tokio::test]
    async fn test_new() {
        let store = Store::new("test_path").unwrap();
        assert!(store.db.is_empty());
    }

    #[tokio::test]
    async fn test_put() {
        let store = Store::new("test_path").unwrap();
        store.put("key".to_string(), "value".to_string()).await.unwrap();
        assert_eq!(store.db.get("key"), Some(&"value".to_string()));
    }

    #[tokio::test]
    async fn test_get() {
        let store = Store::new("test_path").unwrap();
        store.put("key".to_string(), "value".to_string()).await.unwrap();
        let value = store.get("key".to_string()).await.unwrap();
        assert_eq!(value, Some("value".to_string()));
    }
