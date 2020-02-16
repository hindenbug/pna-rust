use clap::{App, Arg};
use structopt::clap::{arg_enum, value_t};
use kvs::{Result, KvsError};
use kvs::{KvsEngine, SledKvsEngine};
use log::{debug, error, info, LevelFilter};
use std::str::FromStr;

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
                .takes_value(true),
        )
        .get_matches();

    let addr: &str = matches.value_of("addr").unwrap_or("127.0.0.0:4000");
    let engine = matches.value_of("engine").unwrap_or("kvs");

    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("kvs-server storage engine {}", engine);
    info!("Listening on {}", addr);
    Ok(())
}
