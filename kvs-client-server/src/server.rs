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

        println!("{:?}", reader);
        let req_reader = serde_json::Deserializer::from_reader(reader).into_iter::<Request>();

        for req in req_reader {
            info!("Received request from {}: {:?}", peer_addr, req);
        }

        Ok(())
    }
}
