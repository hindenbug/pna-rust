use crate::network::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::{KvsEngine, Result};
use log::{debug, error, info};
use serde_json::Deserializer;
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
        let listener = TcpListener::bind(addr).unwrap();
        Server { listener, engine }
    }

    pub fn serve(&mut self) -> Result<()> {
        debug!("Waiting for connections...");
        let listnr = self.listener.try_clone().unwrap();
        for stream in listnr.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.handle_client(stream) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => error!("Connection failed, reason: {:?}", e),
            }
        }
        Ok(())
    }

    fn handle_client(&mut self, stream: TcpStream) -> Result<()> {
        debug!(
            "Connection established from {}, waiting for data...",
            stream.peer_addr()?
        );
        let peer_addr = stream.peer_addr()?;
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

        //TIL macros_rules!
        macro_rules! send_response {
            ($resp:expr) => {{
                let resp = $resp;
                serde_json::to_writer(&mut writer, &resp)?;
                writer.flush()?;
                info!("Response sent to {}: {:?}", peer_addr, resp);
            };};
        }

        for req in req_reader {
            let req = req?;
            debug!("Received request from {}: {:?}", peer_addr, req);
            match req {
                Request::Get { key } => {
                    let engine_response = match self.engine.get(key) {
                        Ok(value) => GetResponse::Ok(value),
                        Err(err) => GetResponse::Err(format!("{}", err)),
                    };
                    send_response!(engine_response);
                }
                Request::Set { key, value } => {
                    let engine_response = match self.engine.set(key, value) {
                        Ok(_) => SetResponse::Ok(()),
                        Err(err) => SetResponse::Err(format!("{}", err)),
                    };
                    send_response!(engine_response);
                }
                Request::Remove { key } => {
                    let engine_response = match self.engine.remove(key) {
                        Ok(_) => RemoveResponse::Ok(()),
                        Err(err) => RemoveResponse::Err(format!("{}", err)),
                    };
                    send_response!(engine_response);
                }
            }
        }

        Ok(())
    }
}
