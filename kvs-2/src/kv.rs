use failure::Fail;
use std::fs::{self};
use std::io;
use std::path::PathBuf;

#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Key not found")]
    KeyNotFound,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are persisted to disk in log files. Log files are named after
/// monotonically increasing generation numbers with a `log` extension name.
/// A `BTreeMap` in memory stores the keys and the value locations for fast query.
///
/// ```rust
/// # use kvs::{KvStore, Result};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// let mut store = KvStore::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct KvStore {
    path: PathBuf,
}

impl KvStore {
    // TIL impl Trait
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        Ok(KvStore { path })
    }

    pub fn set(&mut self, _key: String, _value: String) -> Result<()> {
        Ok(())
    }

    pub fn get(&self, _key: String) -> Result<Option<String>> {
        Ok(None)
    }

    pub fn remove(&mut self, _key: String) -> Result<()> {
        Err(KvsError::KeyNotFound)
    }
}
