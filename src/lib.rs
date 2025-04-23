mod configuration;
mod conway;
mod runner;
mod view;

use anyhow::Error;
pub use configuration::Configuration;
pub use conway::ConwayGame;
pub use runner::Runner;

pub type Res<T> = Result<T, Error>;
