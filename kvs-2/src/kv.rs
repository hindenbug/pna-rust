use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufReader, Seek, SeekFrom, Write};
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
    map: BTreeMap<String, u64>,
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
        let map: BTreeMap<String, u64> = BTreeMap::new();
        fs::create_dir_all(&path)?;
        let log = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(&path.join("log.data"))?;

        // Initialize KvStore
        let mut store = KvStore {
            path: path,
            log: log,
            map: map,
        };

        // Load from log file
        store.load_from_log()?;
        Ok(store)
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command {
            cmd: CommandType::Set,
            key: key.clone(),
            value: Some(value.clone()),
        };
        let current_log_file_offset = self.log.seek(SeekFrom::End(0))?;
        // encoding before writing to log
        serde_json::to_writer(&mut self.log, &command)?;
        self.log.flush()?;
        self.map.insert(key, current_log_file_offset);
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // println!("{:?}", key);
        // println!("{:?}", self.map);
        if let Some(offset) = self.map.get(&key).cloned() {
            &self.log.seek(SeekFrom::Start(offset));
            let mut de = serde_json::Deserializer::from_reader(&self.log);
            let cmd: Command = serde::de::Deserialize::deserialize(&mut de)?;
            Ok(cmd.value)
        } else {
            Ok(None)
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.log.seek(SeekFrom::End(0))?;
        if self.map.get(&key).is_none() {
            return Err(KvsError::KeyNotFound);
        }
        let command = Command {
            cmd: CommandType::Rm,
            key: key.clone(),
            value: None,
        };
        // encoding before writing to log
        serde_json::to_writer(&mut self.log, &command)?;
        self.log.flush()?;
        self.map.remove(&key);
        Ok(())
    }

    fn load_from_log(&mut self) -> Result<()> {
        let mut reader = BufReader::new(File::open(&self.path.join("log.data"))?);
        let mut offset = reader.seek(SeekFrom::Start(0))?;
        let mut stream = serde_json::Deserializer::from_reader(reader).into_iter::<Command>();

        while let Some(cmd) = stream.next() {
            let new_offset = stream.byte_offset() as u64;
            match cmd {
                Ok(Command {
                    cmd: CommandType::Set,
                    key,
                    value: _value,
                }) => {
                    self.map.insert(key, offset);
                }
                Ok(Command {
                    cmd: CommandType::Rm,
                    key,
                    value: None,
                }) => {
                    self.map.remove(&key);
                }
                _ => panic!(),
            }
            offset = new_offset;
        }
        Ok(())
    }
}
