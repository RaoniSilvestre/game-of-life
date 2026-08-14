mod automaton;
mod config;
mod listener;
mod runner;
mod view;

use anyhow::Error;

pub use config::Configuration;
pub use runner::Runner;

pub type Res<T> = Result<T, Error>;
