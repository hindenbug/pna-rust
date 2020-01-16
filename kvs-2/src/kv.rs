use failure::Fail;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self, Write};
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
#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    log: std::fs::File,
}

#[derive(Serialize, Deserialize, Debug)]
enum CommandType {
    Set,
    Rm,
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    cmd: CommandType,
    key: String,
    value: String,
}

impl KvStore {
    // TIL impl Trait
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        let mut log = fs::File::create(&path.join("log.data"))?;

        Ok(KvStore { path, log })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let record = Command {
            cmd: CommandType::Set,
            key: key,
            value: value,
        };
        // binary encoding before writing to log
        let mut encoded_record: Vec<u8> = bincode::serialize(&record).unwrap();
        encoded_record.insert(0, encoded_record.len() as u8);

        self.log.write_all(&encoded_record);
        Ok(())
    }

    pub fn get(&self, _key: String) -> Result<Option<String>> {
        Ok(None)
    }

    pub fn remove(&mut self, _key: String) -> Result<()> {
        Err(KvsError::KeyNotFound)
    }
}
