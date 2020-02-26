use crate::network::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::{KvsError, Result};
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Client {
    pub fn new<T>(addr: T) -> Result<Self>
    where
        T: ToSocketAddrs,
    {
        let reader_stream = TcpStream::connect(addr)?;
        let writer_stream = reader_stream.try_clone()?;
        Ok(Self {
            reader: BufReader::new(reader_stream),
            writer: BufWriter::new(writer_stream),
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;

        let mut deserializer = Deserializer::new(IoRead::new(&mut self.reader));
        let resp = GetResponse::deserialize(&mut deserializer)?;

        match resp {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(err) => Err(KvsError::StringError(err)),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;

        let mut deserializer = Deserializer::new(IoRead::new(&mut self.reader));
        let resp = SetResponse::deserialize(&mut deserializer)?;
        match resp {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(err) => Err(KvsError::StringError(err)),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;

        let mut deserializer = Deserializer::new(IoRead::new(&mut self.reader));
        let resp = RemoveResponse::deserialize(&mut deserializer)?;
        match resp {
            RemoveResponse::Ok(_) => Ok(()),
            RemoveResponse::Err(err) => Err(KvsError::StringError(err)),
        }
    }
}
