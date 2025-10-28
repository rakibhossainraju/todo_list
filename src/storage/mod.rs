#[cfg(feature = "web")]
mod web_storage;

#[cfg(feature = "desktop")]
mod desktop_storage;

mod persist_storage;

pub use persist_storage::PersistStorage;
