use serde_json;
use web_sys::{window, Storage};

#[derive(Clone)]
pub struct PersistStorage {
    storage: web_sys::Storage,
}

impl PersistStorage {
    pub fn new() -> Self {
        let storage = Self::get_local_storage();
        Self { storage }
    }

    pub fn save<T: serde::Serialize>(&self, key: &str, value: &T) {
        let value_str = serde_json::to_string(value).expect("Could not serialize value");
        self.storage.set_item(key, &value_str).expect("Could not set item in local storage");
    }

    pub fn load<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        let value = self.storage.get_item(key).ok()??;
        serde_json::from_str(&value).ok()
    }
    fn get_local_storage() -> Storage {
        window()
            .expect("Could not get the window")
            .local_storage()
            .expect("Could not get local storage")
            .expect("Local storage is not available")
    }
}
