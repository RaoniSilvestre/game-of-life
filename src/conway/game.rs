use super::{Cell, Matrix, Point, State};

#[derive(Debug)]
pub struct ConwayGame {
    pub matrix: Vec<Vec<Cell>>,
    pub size: Point,
}

impl ConwayGame {
    pub fn new(size: Point) -> Self {
        let matrix = Matrix::factory(size);
        ConwayGame { matrix, size }
    }

    pub fn get_alive_cells(matrix: &[Vec<Cell>]) -> Vec<Cell> {
        matrix
            .iter()
            .flatten()
            .filter(|cell| cell.is_alive())
            .copied()
            .collect()
    }

    pub fn start_state(&mut self, living_cells: Vec<Point>) {
        living_cells
            .iter()
            .for_each(|p| self.matrix[p.row][p.col].state = State::Alive);
    }

    pub fn update_living_cells(&mut self) {
        let matrix_in = self.matrix.clone();

        let updater = Cell::update_state_factory(&matrix_in, self.size);

        let mut matrix_out: Vec<Vec<Cell>> = Matrix::factory(self.size);

        matrix_in
            .iter()
            .flatten()
            .for_each(|cell| updater(cell.point, &mut matrix_out));

        self.matrix = matrix_out;
    }
}
