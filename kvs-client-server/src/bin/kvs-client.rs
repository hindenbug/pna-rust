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

use kvs::{Client, Result};
use structopt::StructOpt;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";

#[derive(StructOpt)]
#[structopt(name = "kvs-client")]
struct Options {
    #[structopt(subcommand)]
    sub: SubCommand,
}

#[derive(StructOpt)]
enum SubCommand {
    #[structopt(about = "Set the value of a string key to a string")]
    Set {
        #[structopt(help = "A string key", name = "KEY")]
        key: String,
        #[structopt(help = "The string value of the key", name = "VALUE")]
        value: String,
        #[structopt(
            long="addr", help = "Set the server address",
            value_name = "IP:PORT",
            default_value = DEFAULT_LISTENING_ADDRESS,
            parse(try_from_str)
        )]
        addr: String,
    },
    #[structopt(about = "Get the string value of a given string key")]
    Get {
        #[structopt(help = "A string key", name = "KEY")]
        key: String,
        #[structopt(
            long="addr", help = "Set the server address",
            value_name = "IP:PORT",
            default_value = DEFAULT_LISTENING_ADDRESS,
            parse(try_from_str)
        )]
        addr: String,
    },
    #[structopt(about = "Remove a given key")]
    Rm {
        #[structopt(help = "A string key", name = "KEY")]
        key: String,
        #[structopt(
            long="addr", help = "Set the server address",
            value_name = "IP:PORT",
            default_value = DEFAULT_LISTENING_ADDRESS,
            parse(try_from_str)
        )]
        addr: String,
    },
}

fn main() -> Result<()> {
    let opts = Options::from_args();
    match opts.sub {
        SubCommand::Set { key, value, addr } => {
            let mut client = Client::new(addr)?;
            client.set(key.to_string(), value.to_string())?
        }
        SubCommand::Get { key, addr } => {
            let mut client = Client::new(addr)?;
            let output = match client.get(key.to_string())? {
                Some(value) => value,
                None => "Key not found".to_string(),
            };

            println!("{}", output);
        }
        SubCommand::Rm { key, addr } => {
            let mut client = Client::new(addr)?;
            client.remove(key.to_string())?;
        }
    }
    Ok(())
}
