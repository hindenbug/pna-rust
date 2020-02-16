pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};

mod engines;
mod error;