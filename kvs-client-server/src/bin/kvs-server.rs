use clap::{App, Arg};
use kvs::Result;

fn main() -> Result<()> {
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
    Ok(())
}
