use crate::{KvsEngine, Result};
use log::{debug, error};
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

    pub fn serve(&mut self) -> Result<()> {
        debug!("Waiting for connections...");
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    debug!("Connection established from {}", stream.peer_addr()?);
                }
                Err(e) => error!("Connection failed, reason: {:?}", e),
            }
        }
        Ok(())
    }
}
