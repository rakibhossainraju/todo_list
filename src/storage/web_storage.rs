use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use web_sys::{window, Storage};

#[derive(Clone)]
pub struct WebStorage {
    storage: Storage,
}

impl WebStorage {
    pub fn new() -> Self {
        Self {
            storage: window()
                .expect("Could not get window")
                .local_storage()
                .expect("Could not get local storage")
                .expect("Local storage unavailable"),
        }
    }

    pub fn save<T: Serialize>(&self, key: &str, value: &T) {
        let json = serde_json::to_string(value).expect("Could not serialize value");
        self.storage.set_item(key, &json).expect("Failed to write to localStorage");
    }

    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let value = self.storage.get_item(key).ok()??;
        serde_json::from_str(&value).ok()
    }
}
