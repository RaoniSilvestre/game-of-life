use ratatui::prelude::Color;
use serde::{de::DeserializeOwned, Serialize};

mod conway;
mod firesim;

pub use conway::{Conway, ConwayScene};
pub use firesim::{FireScene, FireSim};

pub trait Automaton: Sized {
    type Cell: Into<Color> + Copy + Default + PartialEq + Send + 'static + Sync;
    type Scene: DeserializeOwned + Serialize;

    fn name() -> &'static str;
    fn new(width: usize, height: usize) -> Self;
    fn from(scene: Self::Scene) -> Self;
    fn board(&self) -> &Board<Self::Cell>;
    fn step(&mut self);
    fn paint_left(&mut self, _p: Point) {}
    fn paint_right(&mut self, _p: Point) {}
    fn iterations(&self) -> usize;
}

pub type Point = (i32, i32);

#[derive(Clone, Debug)]
pub struct Board<C> {
    width: usize,
    height: usize,
    cells: Vec<C>,
}

impl<C: Copy + Default> Board<C> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![C::default(); width * height],
        }
    }

    pub fn get(&self, (x, y): Point) -> C {
        if x < 0 || y < 0 {
            return C::default();
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.height || y >= self.width {
            return C::default();
        }

        self.cells[x * self.width + y]
    }

    pub fn set(&mut self, (x, y): Point, cell: C) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.height || y >= self.width {
            return;
        }

        self.cells[x * self.width + y] = cell;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> &[C] {
        &self.cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fills_with_default() {
        let board = Board::<u8>::new(4, 2);
        assert_eq!(board.cells(), &vec![0; 8]);
    }

    #[test]
    fn test_get_out_of_bounds_returns_default() {
        let board = Board::<u8>::new(5, 5);
        assert_eq!(board.get((-1, 0)), 0);
        assert_eq!(board.get((0, -1)), 0);
        assert_eq!(board.get((5, 0)), 0);
        assert_eq!(board.get((0, 5)), 0);
    }

    #[test]
    fn test_set_out_of_bounds_is_ignored() {
        let mut board = Board::<u8>::new(5, 5);
        board.set((-1, 0), 1);
        board.set((0, -1), 1);
        board.set((5, 0), 1);
        board.set((0, 5), 1);
        assert!(board.cells().iter().all(|&c| c == 0));
    }

    #[test]
    fn test_set_and_get() {
        let mut board = Board::<u8>::new(3, 3);
        board.set((1, 2), 7);
        assert_eq!(board.get((1, 2)), 7);
    }
}
