use super::{Cell, Point};

pub struct Matrix;

impl Matrix {
    pub fn factory(size: Point) -> Vec<Vec<Cell>> {
        let default_cell = Cell::default();
        let (rows, cols) = (size.row, size.col);
        let mut new_cell_matrix = vec![vec![default_cell; cols]; rows];

        for (i, row) in new_cell_matrix.iter_mut().enumerate().take(rows) {
            for (j, cell) in row.iter_mut().enumerate().take(cols) {
                cell.point = Point { row: i, col: j }
            }
        }

        new_cell_matrix
    }
}

#[cfg(test)]
mod matrix_tests {

    use crate::conway::{Cell, Point, State};

    use super::Matrix;

    #[test]
    fn matrix_should_return_that_cell_has_2_neighbours() {
        let matrix = create_5x5_matrix_with_3_living_cells();
        let alive_neighbours_of_2_2 = crate::conway::Neighbour::alive_neighbours(
            &matrix,
            matrix[2][2].point,
            Point { row: 5, col: 5 },
        );
        assert_eq!(alive_neighbours_of_2_2, 2);
    }

    #[test]
    fn matrix_should_return_that_cell_has_1_neighbours() {
        let matrix = create_5x5_matrix_with_3_living_cells();

        let alive_neighbours_of_2_2 = crate::conway::Neighbour::alive_neighbours(
            &matrix,
            matrix[3][2].point,
            Point { row: 5, col: 5 },
        );
        assert_eq!(alive_neighbours_of_2_2, 1);
    }

    fn create_5x5_matrix_with_3_living_cells() -> Vec<Vec<Cell>> {
        let lista_de_vivos: Vec<Point> = vec![
            Point { row: 1, col: 2 },
            Point { row: 2, col: 2 },
            Point { row: 3, col: 2 },
        ];

        let mut matrix: Vec<Vec<Cell>> = Matrix::factory(Point { row: 5, col: 5 });

        for vivo in lista_de_vivos {
            matrix[vivo.row][vivo.col] = Cell::new(
                Point {
                    row: vivo.row,
                    col: vivo.col,
                },
                State::Alive,
            );
        }

        matrix
    }
}
