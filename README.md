# Conway's game of life

![GitHub repo size](https://img.shields.io/github/repo-size/RaoniSilvestre/game-of-life?style=for-the-badge)
![GitHub language count](https://img.shields.io/github/languages/count/RaoniSilvestre/game-of-life?style=for-the-badge)


https://github.com/user-attachments/assets/276e1ab8-ee08-435f-a7e7-de248b5e04ef

A simple and interactive terminal-based cellular automaton simulator.
It ships with Conway's Game of Life, with a generic engine: any cellular automaton
can be added by implementing a single trait, and each simulation is driven by a scene
file (TOML) that defines the grid, initial state and speed.

## Controls

| Key/Action | Effect |
|:---|:---|
| Left Mouse Button | Toggle cell state (left button action) |
| Right Mouse Button | Secondary action (Conway: scatter 16 random cells) |
| Enter | Start or pause the simulation |
| q | Quit the game |

## Running the project

You can run the project either with Cargo or inside a Docker container.

### Running with Cargo

Install rust and cargo: https://www.rust-lang.org/tools/install

```rust 
cargo run --release -q
```

You can also pass options:

```rust 
cargo run -q -- -f 10
cargo run -q -- --scene scenes/conway.toml
```

### Running with Docker

First, build the image:

```
docker build -t game-of-life .
```

Then run it interactively:

```
docker run -it game-of-life
```

You can also pass flags, just like when running directly:

```
docker run -it game-of-life -f 10
```

## CLI Help

```
❯ cargo run -q -- --help
Usage: game-of-life [OPTIONS]

Options:
  -f, --fps <FPS>      Frames per second (overrides the scene)
      --scene <SCENE>  Path to the scene file [default: scenes/conway.toml]
  -h, --help           Print help
  -V, --version        Print version
```

## Scene files

Each simulation is defined by a TOML scene file:

```toml
name = "Conway's Game of Life"   # title shown on screen
automaton = "conway"             # which automaton to run (see below)
width = 40                       # grid width in cells
height = 20                      # grid height in cells
fps = 1                          # optional; CLI --fps overrides this

initial = [                      # optional; each char becomes a cell
    "....X...",
    "...XXX..",
]
```

Cells are interpreted by each automaton. For Conway, `X`/`x`/`█`/`O` mean "alive" and
anything else (usually `.`) means "dead". An empty `initial` starts with a blank grid
so you can draw with the mouse.

## Adding a new automaton

Implement the `Automaton` trait (`src/automaton.rs`) and register it in
`run_with_scene` (`src/runner.rs`):

```rust
pub trait Automaton {
    type Cell: Into<Color> + Copy + Default + PartialEq + Send + 'static;

    fn name() -> &'static str;
    fn new(width: usize, height: usize) -> Self;
    fn from_rows(width: usize, height: usize, rows: &[String]) -> Res<Self>;
    fn board(&self) -> &Board<Self::Cell>;
    fn step(&mut self);
    fn paint_left(&mut self, x: i32, y: i32);   // default: no-op
    fn paint_right(&mut self, x: i32, y: i32);  // default: no-op
    fn iterations(&self) -> usize;
}
```

The associated `Cell` type is the automaton's own state enum — support for N states
comes from the type itself, and coloring is defined by each automaton via
`impl From<Cell> for Color`. That is what makes disease, fire and other multi-state
simulations possible without any changes to the engine.

## About the project

This project is a fun exploration of terminal graphics and event handling using Rust.
It focuses on a smooth user experience with real-time interaction and a simple CLI interface.

