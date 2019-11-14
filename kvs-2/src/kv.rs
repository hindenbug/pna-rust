use failure::Fail;
use std::path::PathBuf;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "Key not found")]
    KeyNotFound,
}

pub type Result<T> = std::result::Result<T, KvsError>;

#[derive(Default)]
pub struct KvStore {}

impl KvStore {
    // TIL impl Trait
    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        panic!()
    }

    pub fn set(&mut self, _key: String, _value: String) -> Result<()> {
        panic!()
    }

    pub fn get(&self, _key: String) -> Result<Option<String>> {
        panic!()
    }

    pub fn remove(&mut self, _key: String) -> Result<()> {
        panic!()
    }
}
