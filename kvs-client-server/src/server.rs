use crate::network::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::{KvsEngine, Result};
use log::{debug, error, info};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct Server<E: KvsEngine> {
    listener: TcpListener,
    engine: E,
}

impl<E: KvsEngine> Server<E> {
    // TIL generic types
    // Defines a function `new` that takes a generic type `T` which
    // must implement trait `ToSocketAddr`.
    pub fn new<T>(addr: T, engine: E) -> Self
    where
        T: ToSocketAddrs,
    {
        Server {
            listener: TcpListener::bind(addr).unwrap(),
            engine,
        }
    }

    pub fn serve(mut self) -> Result<()> {
        debug!("Waiting for connections...");
        let listener = self.listener.try_clone()?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream)?;
                }
                Err(e) => error!("Connection failed, reason: {:?}", e),
            }
        }
        Ok(())
    }

    pub fn handle_client(&mut self, stream: TcpStream) -> Result<()> {
        debug!("Waiting data from {}", stream.peer_addr()?);
        let peer_addr = stream.peer_addr()?;
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        let req_reader = serde_json::Deserializer::from_reader(reader).into_iter::<Request>();

        macro_rules! send_response {
            ($resp:expr) => {
                let resp = $resp;
                serde_json::to_writer(&mut writer, &resp)?;
                writer.flush()?;
                info!("Response sent to {}: {:?}", peer_addr, resp);
            };
        }

        for req in req_reader {
            debug!("Received request from {}: {:?}", peer_addr, req);
            match req? {
                Request::Get { key } => {
                    debug!("key: {}", key);
                    send_response!(GetResponse::Ok(Some("val".to_string())));
                }
                Request::Set { key, value } => {
                    debug!("key: {}, value: {}", key, value);
                    send_response!(SetResponse::Ok(()));
                }
                Request::Remove { key } => {
                    debug!("key: {}", key);
                    send_response!(RemoveResponse::Ok(()));
                }
            }
        }

        Ok(())
    }
}
