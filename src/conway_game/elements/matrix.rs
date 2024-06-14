use super::cell::{Cell, State};
use super::point::Point;
pub struct CellMatrix;

impl CellMatrix {
    pub fn new(size: Point) -> Vec<Vec<Cell>> {
        let default_cell = Cell::new_default();

        let (rows, cols) = (size.row, size.col);

        let mut new_cell_matrix = vec![vec![default_cell; cols.into()]; rows.into()];

        for i in 0..rows {
            for j in 0..cols {
                new_cell_matrix[i][j].point = Point { row: i, col: j }
            }
        }

        new_cell_matrix
    }

    pub fn new_with_states(list_of_alive_cell_points: Vec<Point>, size: Point) -> Vec<Vec<Cell>> {
        let mut new_cell_matrix = Self::new(size);

        for point in list_of_alive_cell_points.iter() {
            new_cell_matrix[point.row][point.col].state = State::Alive;
        }

        new_cell_matrix
    }

    pub fn print_matrix(matrix: Vec<Vec<Cell>>) {
        for line in matrix {
            for cell in line {
                print!(" {:?} ", cell.state)
            }
            println!();
        }
    }
}
