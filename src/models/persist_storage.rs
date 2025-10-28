use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use web_sys::{window, Storage};

/// Trait that defines the interface for persistent storage implementations
/// This allows different storage backends (web, desktop, etc.) to be used interchangeably
pub trait StorageHandlers {
    fn new() -> Self;
    fn save<T: Serialize>(&self, key: &str, value: &T);
    fn load<T: DeserializeOwned>(&self, key: &str) -> Option<T>;
}

#[derive(Clone)]
pub struct PersistStorage {
    storage: web_sys::Storage,
}

impl StorageHandlers for PersistStorage {
    fn new() -> Self {
        let storage = Self::get_local_storage();
        Self { storage }
    }

    fn save<T: Serialize>(&self, key: &str, value: &T) {
        let value_str = serde_json::to_string(value).expect("Could not serialize value");
        self.storage.set_item(key, &value_str).expect("Could not set item in local storage");
    }

    fn load<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let value = self.storage.get_item(key).ok()??;
        serde_json::from_str(&value).ok()
    }
}

impl PersistStorage {
    fn get_local_storage() -> Storage {
        window()
            .expect("Could not get the window")
            .local_storage()
            .expect("Could not get local storage")
            .expect("Local storage is not available")
    }
}
