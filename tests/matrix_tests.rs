#[cfg(test)]
mod matrix_tests {
    use gol::game::elements::{Cell, CellMatrix, Neighbour, Point};

    #[test]
    fn matrix_should_return_that_cell_has_2_neighbours() {
        let matrix = create_5x5_matrix_with_3_living_cells();
        let alive_neighbours_of_2_2 = Neighbour::alive_neighbours(matrix[2][2].point, &matrix);
        assert_eq!(alive_neighbours_of_2_2, 2);
    }

    #[test]
    fn matrix_should_return_that_cell_has_1_neighbours() {
        let matrix = create_5x5_matrix_with_3_living_cells();

        let alive_neighbours_of_2_2 = Neighbour::alive_neighbours(matrix[3][2].point, &matrix);
        assert_eq!(alive_neighbours_of_2_2, 1);
    }

    fn create_5x5_matrix_with_3_living_cells() -> Vec<Vec<Cell>> {
        let lista_de_vivos: Vec<Point> = vec![
            Point { row: 1, col: 2 },
            Point { row: 2, col: 2 },
            Point { row: 3, col: 2 },
        ];

        let matrix: Vec<Vec<Cell>> =
            CellMatrix::new_with_states(lista_de_vivos, Point { row: 5, col: 5 });

        matrix
    }
}
