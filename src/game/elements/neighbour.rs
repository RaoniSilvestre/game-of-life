use super::{Cell, Neighbour, Point, State};

impl Neighbour {
    // Neighbours logic
    pub fn alive_neighbours(point: Point, matrix: &[Vec<Cell>]) -> usize {
        let mut alives = 0;

        let verify_boundaries = Self::is_inside_bounds_factory(matrix);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor_row = (point.row as i32) + dx;
                let neighbor_col = (point.col as i32) + dy;

                if verify_boundaries((neighbor_row, neighbor_col)) {
                    if matrix[neighbor_row as usize][neighbor_col as usize].state == State::Alive {
                        alives += 1;
                    }
                }
            }
        }

        alives
    }

    fn is_inside_bounds_factory(matrix: &[Vec<Cell>]) -> impl Fn((i32, i32)) -> bool + '_ {
        move |x: (i32, i32)| Self::is_inside_bounds(matrix, x)
    }

    fn is_inside_bounds(matrix: &[Vec<Cell>], (neighbor_row, neighbor_col): (i32, i32)) -> bool {
        if neighbor_col < 0
            || neighbor_row < 0
            || neighbor_row >= matrix.len() as i32
            || neighbor_col >= matrix[0].len() as i32
        {
            return false;
        }

        true
    }
}
