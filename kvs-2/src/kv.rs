use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::env::current_dir;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error, Write};
use std::path::PathBuf;

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
    log: File,
    map: BTreeMap<String, String>,
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
    value: Option<String>,
}

impl KvStore {
    // TIL impl Trait
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        fs::create_dir_all(&path)?;
        let mut log = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(&path.join("log.data"))?;

        let reader = BufReader::new(File::open(&path.join("log.data"))?);
        let mut stream = serde_json::Deserializer::from_reader(reader).into_iter::<Command>();

        while let Some(cmd) = stream.next() {
            match cmd {
                Ok(Command {
                    cmd: CommandType::Set,
                    key: key,
                    value: Some(value),
                }) => {
                    map.insert(key, value);
                }
                Ok(Command {
                    cmd: CommandType::Rm,
                    key: key,
                    value: None,
                }) => {
                    map.remove(&key);
                }
                _ => panic!(),
            }
        }

        Ok(KvStore {
            path: path,
            log: log,
            map: map,
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command {
            cmd: CommandType::Set,
            key: key.clone(),
            value: Some(value.clone()),
        };
        // encoding before writing to log
        serde_json::to_writer(&mut self.log, &command);
        self.log.flush()?;
        self.map.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
        // let stored_value = self.map.get(&key);
        // match stored_value {
        //     Some(value) => return Ok(Some(value.to_string())),
        //     None => return Ok(None),
        // };
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.map.get(&key).is_none() {
            return Err(KvsError::KeyNotFound);
        }
        let command = Command {
            cmd: CommandType::Rm,
            key: key.clone(),
            value: None,
        };
        // encoding before writing to log
        serde_json::to_writer(&mut self.log, &command);
        self.map.remove(&key);
        Ok(())
    }
}
