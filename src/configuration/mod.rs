pub mod config;
pub mod parser;

use crate::game::elements::Point;
use clap::{Parser, ValueEnum};
#[derive(Debug)]
pub struct Config {
    pub size: Point,
    pub char: char,
    pub fps: u64,
    pub mode: Mode,
    pub rand_points: usize,
    pub duration: Option<usize>,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// Start in test mode
    Test,
    /// Start in random mode
    Random,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Modo de inicialização do jogo
    #[arg(short, long, default_value_t = Mode::Random)]
    pub mode: Mode,

    /// Tamanho do eixo X do jogo
    #[arg(short = 'x', long, default_value_t = 100)]
    pub dx: usize,

    /// Tamanho do eixo Y do jogo
    #[arg(short = 'y', long, default_value_t = 30)]
    pub dy: usize,

    /// Frames por segundo
    #[arg(short, long, default_value_t = 10)]
    pub fps: u64,

    /// Quantidade de bolinhas aleatórias, usado apenas no modo "random"
    #[arg(short, long, default_value_t = 3000)]
    pub random: usize,

    /// Caractere que apareçerá na tela como célula viva
    #[arg(short = 'c', long, default_value_t = '#')]
    pub def_char: char,

    /// Duração do jogo
    #[arg(short = 'd', long)]
    pub duration: Option<usize>,
}
