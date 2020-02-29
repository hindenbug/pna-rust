use crate::KvsEngine;
use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;

const COMPACTION_THRESHOLD: u64 = 1024;
const LOG_FILE_NAME: &str = "current.db";

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    log: File,
    map: BTreeMap<String, LogPointer>,
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

#[derive(Debug)]
struct LogPointer {
    offset: u64,
    len: u64,
}

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let offset = self.log.seek(SeekFrom::End(0))?;
        let command = Command {
            cmd: CommandType::Set,
            key: key.clone(),
            value,
        };
        // encoding before writing to log
        serde_json::to_writer(&mut self.log, &command)?;
        self.log.flush()?;
        let current_offset = self.log.seek(SeekFrom::End(0))?;
        self.map.insert(
            key,
            LogPointer {
                offset,
                len: current_offset - offset,
            },
        );
        if current_offset > COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(pointer) = self.map.get(&key) {
            &self.log.seek(SeekFrom::Start(pointer.offset));
            let mut de = serde_json::Deserializer::from_reader(&self.log);
            let cmd: Command = serde::de::Deserialize::deserialize(&mut de)?;
            Ok(Some(cmd.value))
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.log.seek(SeekFrom::End(0))?;
        if self.map.get(&key).is_none() {
            return Err(KvsError::KeyNotFound);
        }
        let command = Command {
            cmd: CommandType::Rm,
            key: key.clone(),
            value: String::new(),
        };
        // encoding before writing to log
        serde_json::to_writer(&mut self.log, &command)?;
        self.log.flush()?;
        self.map.remove(&key);
        Ok(())
    }
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        let log = Self::new_log_file(&path)?;

        let mut store = KvStore {
            path: path,
            log: log,
            map: BTreeMap::new(),
        };

        // Load from log files
        store.load_from_log()?;
        Ok(store)
    }

    fn load_from_log(&mut self) -> Result<()> {
        let mut reader = BufReader::new(File::open(&self.path.join(LOG_FILE_NAME))?);
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
                    self.map.insert(
                        key,
                        LogPointer {
                            offset,
                            len: new_offset - offset,
                        },
                    );
                }
                Ok(Command {
                    cmd: CommandType::Rm,
                    key,
                    value: _value,
                }) => {
                    self.map.remove(&key);
                }
                _ => panic!(),
            }
            offset = new_offset;
        }
        Ok(())
    }

    fn compact(&mut self) -> Result<()> {
        let tmp_path: PathBuf = self.path.join("tmp.db");
        let file_path: PathBuf = self.path.join(LOG_FILE_NAME);
        let mut work_file = self.log.try_clone()?;

        let mut new_writer = BufWriter::new(
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&tmp_path)?,
        );

        for (_, pointer) in self.map.iter() {
            work_file.seek(SeekFrom::Start(pointer.offset))?;
            let mut de = serde_json::Deserializer::from_reader(&work_file);
            let cmd: Command = serde::de::Deserialize::deserialize(&mut de)?;
            serde_json::to_writer(&mut new_writer, &cmd)?;
        }

        // TODO
        // handle writes during or file locking, during compaction
        // keep log/backup of file pre/post compaction, maybe by max file size
        // fs::rename(work_file, self.path.join(format!("log-{}.db",i)))?;
        fs::rename(tmp_path, file_path)?;
        Ok(())
    }

    fn log_path(path: &PathBuf, fname: String) -> PathBuf {
        path.join(format!("{}", fname))
    }

    fn new_log_file(path: &PathBuf) -> Result<File> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(Self::log_path(&path, LOG_FILE_NAME.to_string()))?;

        Ok(file)
    }
}
