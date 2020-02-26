use clap::{App, Arg};
use kvs::{KvStore, KvsEngine, KvsError, Result, Server, SledKvsEngine};
use log::{info, LevelFilter};
use std::env;
use std::str::FromStr;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Engine {
    Kvs,
    Sled,
}

impl FromStr for Engine {
    type Err = KvsError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "kvs" => Ok(Engine::Kvs),
            "sled" => Ok(Engine::Sled),
            _ => Err(KvsError::EngineNotFound),
        }
    }
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Kvs server starting up.");

    let matches = App::new("kvs-client")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .help("address to listen")
                .takes_value(true)
                .value_name("IP:PORT"),
        )
        .arg(
            Arg::with_name("engine")
                .help("storage engine to use")
                .long("engine")
                .value_name("ENGINE-NAME")
                .case_insensitive(true)
                .takes_value(true),
        )
        .get_matches();

    let addr: &str = matches
        .value_of("addr")
        .unwrap_or(DEFAULT_LISTENING_ADDRESS);
    let engine: Engine = Engine::from_str(matches.value_of("engine").unwrap_or("kvs"))?;

    info!(
        "kvs-server version: {}, storage engine {:?}",
        env!("CARGO_PKG_VERSION"),
        engine
    );
    info!("Listening on: {}", addr.clone());
    match engine {
        Engine::Kvs => start_server_with(addr, KvStore::open(env::current_dir()?)?),
        Engine::Sled => start_server_with(addr, SledKvsEngine::open(env::current_dir()?)?),
    }
}

fn start_server_with<E: KvsEngine>(addr: &str, engine: E) -> Result<()> {
    let mut server = Server::new(addr, engine);
    server.serve()?;
    Ok(())
}
