use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "web")]
use crate::storage::web_storage::WebStorage;

#[cfg(feature = "desktop")]
use crate::storage::desktop_storage;
#[cfg(feature = "desktop")]
use std::marker::PhantomData;

#[derive(Clone)]
pub struct PersistStorage {
    #[cfg(feature = "web")]
    storage: WebStorage,

    #[cfg(feature = "desktop")]
    _marker: PhantomData<()>,
}

impl PersistStorage {
    pub fn new() -> Self {
        #[cfg(feature = "web")]
        {
            return Self { storage: WebStorage::new() };
        }

        #[cfg(feature = "desktop")]
        {
            use std::marker::PhantomData;

            return Self { _marker: PhantomData };
        }
        panic!("No platform feature (web/desktop) enabled for PersistStorage");
    }

    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        #[cfg(feature = "web")]
        return self.storage.load::<T>(key);

        #[cfg(feature = "desktop")]
        return desktop_storage::load(key);

        panic!("No platform feature (web/desktop) enabled for PersistStorage");
    }

    pub fn save<T: Serialize>(&self, key: &str, value: &T) {
        #[cfg(feature = "web")]
        return self.storage.save(key, value);

        #[cfg(feature = "desktop")]
        return desktop_storage::save(key, value);

        panic!("No platform feature (web/desktop) enabled for PersistStorage");
    }
}
