mod automaton;
mod config;
mod listener;
mod runner;
mod view;

pub use automaton::{Automaton, Conway, ConwayScene};
pub use config::Configuration;
pub use runner::Runner;

pub type Res<T> = Result<T, anyhow::Error>;
