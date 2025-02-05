use super::{Cell, Point};

pub struct Matrix;

impl Matrix {
    pub fn factory(size: Point) -> Vec<Vec<Cell>> {
        let mut new_cell_matrix = Self::default((size.row, size.col));

        new_cell_matrix
            .iter_mut()
            .enumerate()
            .for_each(Self::map_row);

        new_cell_matrix
    }

    pub fn map_row((i, row): (usize, &mut Vec<Cell>)) {
        row.iter_mut()
            .enumerate()
            .for_each(|(j, cell)| cell.point = Point { row: i, col: j });
    }

    fn default((row, col): (usize, usize)) -> Vec<Vec<Cell>> {
        vec![vec![Cell::default(); col]; row]
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
