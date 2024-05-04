use super::cell::{Cell, State};
use super::point::Point;
pub struct CellMatrix;

impl CellMatrix {
    pub fn new_with_size(size: Point) -> Vec<Vec<Cell>> {
        let default_cell = Cell::new(Point { row: 0, col: 0 }, State::Dead);

        let (rows, cols) = (size.row, size.col);

        let mut new_cell_matrix = vec![vec![default_cell; cols.into()]; rows.into()];

        for i in 0..rows {
            for j in 0..cols {
                new_cell_matrix[i as usize][j as usize].point = Point { row: i, col: j }
            }
        }

        new_cell_matrix
    }
}
