mod automaton;
mod config;
mod listener;
mod runner;
mod scene;
mod view;

use anyhow::Error;

pub use automaton::{Automaton, Board, Point, Snapshot};
pub use config::Configuration;
pub use runner::{Runner, run_with_scene};
pub use scene::Scene;

pub type Res<T> = Result<T, Error>;
