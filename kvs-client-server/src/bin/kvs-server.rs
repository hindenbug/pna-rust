use kvs::{KvStore, KvsEngine, KvsError, Result, Server, SledKvsEngine};
use log::{info, LevelFilter};
use std::env;
use std::fs;
use std::process::exit;
use std::str::FromStr;
use structopt::StructOpt;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const CONFIG_FILENAME: &str = "engine";

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs-server")]
struct Options {
    #[structopt(long, value_name = "IP:PORT", default_value = DEFAULT_LISTENING_ADDRESS, parse(try_from_str))]
    addr: String,
    #[structopt(
        long,
        help = "Sets the storage engine",
        value_name = "ENGINE-NAME",
        default_value = "kvs"
    )]
    engine: String,
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Kvs server starting up.");

    let opts = Options::from_args();
    let curr_dir = env::current_dir()?;
    let engine_config = curr_dir.join(CONFIG_FILENAME);
    let engine = Engine::from_str(&opts.engine)?;
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
    info!("Listening on: {}", opts.addr.clone());

    fs::write(&engine_config, format!("{}", opts.engine))?;

    match curr_engine {
        Engine::Kvs => start_server_with(&opts.addr, KvStore::open(curr_dir)?),
        Engine::Sled => start_server_with(&opts.addr, SledKvsEngine::open(curr_dir)?),
    }
}

fn start_server_with<E: KvsEngine>(addr: &str, engine: E) -> Result<()> {
    let server = Server::new(addr, engine);
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
