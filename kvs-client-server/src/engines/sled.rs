use crate::{KvsEngine, KvsError, Result};
use sled::Db;
use std::path::PathBuf;

#[derive(Clone)]
pub struct SledKvsEngine {
    tree: Db,
}

impl SledKvsEngine {
    pub fn open(path: impl Into<PathBuf>) -> Result<SledKvsEngine> {
        let tree = sled::open(path.into())?;
        Ok(SledKvsEngine { tree })
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&self, key: String, value: String) -> Result<()> {
        self.tree.insert(key, value.as_bytes())?;
        self.tree.flush()?;
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self
            .tree
            .get(key)?
            .map(|v| v.to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&self, key: String) -> Result<()> {
        self.tree.remove(key)?.ok_or(KvsError::KeyNotFound)?;
        self.tree.flush()?;
        Ok(())
    }
}
