# Conway's game of life

![GitHub repo size](https://img.shields.io/github/repo-size/RaoniSilvestre/game-of-life?style=for-the-badge)
![GitHub language count](https://img.shields.io/github/languages/count/RaoniSilvestre/game-of-life?style=for-the-badge)


https://github.com/user-attachments/assets/276e1ab8-ee08-435f-a7e7-de248b5e04ef

A simple and interactive terminal-based version of Conway's Game of Life, written in Rust!
Customize the speed, draw your initial cells, and watch how the simulation evolves.

## Controls

| Key/Action | Effect |
|:---|:---|
| Left Mouse Button | Toggle cell state (alive/dead) |
| Enter | Start or pause the simulation |
| q | Quit the game |

## Running the project

You can run the project either with Cargo or inside a Docker container.

### Running with Cargo

> ⚡ Note: The first build might take a few minutes as it downloads and compiles all dependencies.

Make sure you have Rust installed: https://www.rust-lang.org/tools/install

```rust 
cargo run --release -q
```

You can also pass options:

```rust 
cargo run -q -- -f 10
```

### Running with Docker

> ⚡ Note: The first Docker build can take a few minutes too, due to Rust compilation.

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
Basic simulator for Conway's Game of Life

Usage: game-of-life [OPTIONS]

Options:
  -f, --fps <FPS>  Frames per second [default: 1]
  -h, --help       Print help
  -V, --version    Print version
```

## About the project

This project is a fun exploration of terminal graphics and event handling using Rust.
It focuses on a smooth user experience with real-time interaction and a simple CLI interface.

Contributions, feedback, and ideas are very welcome! 🚀
