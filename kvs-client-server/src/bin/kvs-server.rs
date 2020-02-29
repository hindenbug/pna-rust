use clap::{App, Arg};
use kvs::{KvStore, KvsEngine, KvsError, Result, Server, SledKvsEngine};
use log::{error, info, LevelFilter};
use std::env;
use std::fs;
use std::process::exit;
use std::str::FromStr;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const CONFIG_FILENAME: &str = "engine";

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

    let matches = App::new("kvs-server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .value_name("ENGINE-NAME")
                .possible_values(&["kvs", "sled"])
                .case_insensitive(true)
                .help("Specify engine [default: kvs]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .value_name("IP-PORT")
                .help("Specify listening address")
                .takes_value(true),
        )
        .get_matches();

    let curr_dir = env::current_dir()?;
    let engine_config = curr_dir.join(CONFIG_FILENAME);
    let addr: &str = matches
        .value_of("addr")
        .unwrap_or(DEFAULT_LISTENING_ADDRESS);

    let engine_option = matches.value_of("engine").unwrap_or("kvs");

    let engine = Engine::from_str(engine_option)?;
    let curr_engine = match previous_engine_type()? {
        Some(prev_engine) => {
            if engine != prev_engine {
                exit(1);
            } else {
                prev_engine
            }
        }
        None => engine,
    };

    info!(
        "kvs-server version: {}, storage engine {:?}",
        env!("CARGO_PKG_VERSION"),
        curr_engine
    );
    info!("Listening on: {}", addr.clone());

    fs::write(&engine_config, format!("{}", engine_option))?;

    match curr_engine {
        Engine::Kvs => start_server_with(addr, KvStore::open(curr_dir)?),
        Engine::Sled => start_server_with(addr, SledKvsEngine::open(curr_dir)?),
    }
}

fn start_server_with<E: KvsEngine>(addr: &str, engine: E) -> Result<()> {
    let mut server = Server::new(addr, engine);
    server.serve()?;
    Ok(())
}

fn previous_engine_type() -> Result<Option<Engine>> {
    let engine_config_path = env::current_dir()?.join(CONFIG_FILENAME);
    if !engine_config_path.exists() {
        Ok(None)
    } else {
        match fs::read_to_string(engine_config_path) {
            Ok(engine) => Ok(Some(Engine::from_str(&engine)?)),
            Err(_) => Ok(None),
        }
    }
}
