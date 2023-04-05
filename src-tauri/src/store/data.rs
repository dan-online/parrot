use sled::{Config, Db};
use tauri::PathResolver;

pub struct DataStore {
    db: Db,
}

static mut DATA_STORE: Option<DataStore> = None;

impl DataStore {
    pub fn new(resolver: PathResolver) -> Self {
        let db = Self::get_db(resolver);
        Self { db }
    }

    pub fn get<T>(&self, key: &str) -> Result<Option<T>, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        if let Some(value) = self.db.get(key)? {
            let value = serde_json::from_slice(&value)?;
            Ok(value)
        } else {
            Err("No value found".into())
        }
    }

    pub fn set<T>(&self, key: &str, value: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_vec(&value)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    fn get_db(resolver: PathResolver) -> Db {
        if let Some(store) = unsafe { DATA_STORE.as_ref() } {
            return store.db.clone();
        }

        let app_dir = resolver.app_data_dir().unwrap();

        let db_path = app_dir.join("parrot_data.db");

        let config = Config::default().path(db_path).cache_capacity(100_000_000); // 100 MB

        let db = config.open().unwrap();

        unsafe {
            DATA_STORE = Some(Self { db: db.clone() });
        }

        db
    }
}
