use super::{Cell, Point, State};

pub struct Neighbour;

impl Neighbour {
    pub fn alive_neighbours(matrix: &[Vec<Cell>], point: Point, size: Point) -> usize {
        let mut alives = 0;

        let x = point.row as i32;
        let y = point.col as i32;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbor_row = ((x + dx).rem_euclid(size.row as i32)) as usize;
                let neighbor_col = ((y + dy).rem_euclid(size.col as i32)) as usize;

                if matrix[neighbor_row][neighbor_col].state == State::Alive {
                    alives += 1;
                }
            }
        }

        alives
    }
}
