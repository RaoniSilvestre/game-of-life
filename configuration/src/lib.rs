pub mod config;
pub mod parser;

use clap::{Parser, ValueEnum};
use game::elements::Point;
#[derive(Debug, Clone, Copy)]
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
    Test,
    Random,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Modo de inicialização do jogo
    #[arg(short, long, default_value_t = Mode::Random)]
    pub mode: Mode,

    /// Tamanho do eixo X do jogo
    #[arg(short = 'x', long)]
    pub dx: Option<usize>,

    /// Tamanho do eixo Y do jogo
    #[arg(short = 'y', long)]
    pub dy: Option<usize>,

    /// Frames por segundo
    #[arg(short, long, default_value_t = 1)]
    pub fps: u64,

    /// Quantidade de bolinhas aleatórias, usado apenas no modo "random"
    #[arg(short, long, default_value_t = 2000)]
    pub random: usize,

    /// Caractere que apareçerá na tela como célula viva
    #[arg(short = 'c', long, default_value_t = '#')]
    pub def_char: char,

    /// Duração do jogo
    #[arg(short = 'd', long)]
    pub duration: Option<usize>,
}
