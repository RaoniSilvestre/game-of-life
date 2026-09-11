mod automaton;
mod config;
mod runner;
mod view;

use anyhow::{bail, Context};
use toml::Value;

pub use automaton::{Automaton, Conway, ConwayScene, FireScene, FireSim};
pub use config::Configuration;
pub use runner::Runner;

pub type Res<T> = Result<T, anyhow::Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomatonType {
    Conway,
    FireSim,
}

impl TryFrom<&Value> for AutomatonType {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let name = value
            .as_str()
            .context("o campo `type` deve ser uma string")?;

        match name {
            "conway" => Ok(Self::Conway),
            "sfb" => Ok(Self::FireSim),
            other => bail!("autômato desconhecido: {other:?}"),
        }
    }
}
