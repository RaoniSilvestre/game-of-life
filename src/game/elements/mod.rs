mod cell;
pub mod matrix;
pub mod neighbour;
pub mod point;

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub state: State,
    pub point: Point,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

pub struct Neighbour;

pub struct CellMatrix;
