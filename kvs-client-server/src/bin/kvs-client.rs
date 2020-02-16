//! The kvs-client executable supports the following command line arguments:
//!
//!     kvs-client set <KEY> <VALUE> [--addr IP-PORT]
//!
//!     Set the value of a string key to a string.
//!     --addr accepts an IP address, either v4 or v6, and a port number, with the format IP:PORT. If --addr is not specified then connect on 127.0.0.1:4000.
//!     Print an error and return a non-zero exit code on server error, or if IP-PORT does not parse as an address.
//!
//!     kvs-client get <KEY> [--addr IP-PORT]
//!     Get the string value of a given string key.
//!     --addr accepts an IP address, either v4 or v6, and a port number, with the format IP:PORT. If --addr is not specified then connect on 127.0.0.1:4000.
//!     Print an error and return a non-zero exit code on server error, or if IP-PORT does not parse as an address.
//!
//!     kvs-client rm <KEY> [--addr IP-PORT]
//!     Remove a given string key.
//!     --addr accepts an IP address, either v4 or v6, and a port number, with the format IP:PORT. If --addr is not specified then connect on 127.0.0.1:4000.
//!     Print an error and return a non-zero exit code on server error, or if IP-PORT does not parse as an address. A "key not found" is also treated as an error in the "rm" command.
//!
//!     kvs-client -V
//!     Print the version.
//! All error messages should be printed to stderr.

use clap::{App, Arg, SubCommand};
use kvs::Result;
use std::net::TcpStream;
use std::process::exit;

fn main() -> Result<()> {
    let matches = App::new("kvs-client")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(
                    Arg::with_name("KEY")
                        .help("The key name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .help("The value")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("addr")
                        .long("addr")
                        .help("address to connect to server")
                        .takes_value(true)
                        .value_name("IP:PORT")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get a value for a key.")
                .arg(
                    Arg::with_name("KEY")
                        .help("Key name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("addr")
                        .long("addr")
                        .help("address to connect to server")
                        .takes_value(true)
                        .value_name("IP:PORT")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(
                    Arg::with_name("KEY")
                        .help("Key name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("addr")
                        .long("addr")
                        .help("address to connect to server")
                        .takes_value(true)
                        .value_name("IP:PORT")
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4000");
            TcpStream::connect(addr)?;
        }
        ("get", Some(matches)) => {
            let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4000");
            TcpStream::connect(addr)?;
        }
        ("rm", Some(matches)) => {
            let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4000");
            TcpStream::connect(addr)?;
        }
        _ => {
            eprintln!("Command Not Found");
            exit(1)
        }
    }
    Ok(())
}
