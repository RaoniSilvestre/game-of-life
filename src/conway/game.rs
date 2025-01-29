use super::{Cell, Matrix, Point, State};

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
        let mut alive_cells = Vec::new();
        for row in matrix.iter() {
            for cell in row.iter() {
                if cell.state == State::Alive {
                    alive_cells.push(*cell)
                }
            }
        }
        alive_cells
    }

    pub fn start_state(&mut self, points_of_living_cells: Vec<Point>) {
        for point in points_of_living_cells {
            self.matrix[point.row][point.col].state = State::Alive;
        }
    }

    pub fn update_living_cells(&mut self) {
        let matrix_in = self.matrix.clone();
        let update_state = Cell::update_state_factory(&matrix_in, self.size);
        let mut matrix_out: Vec<Vec<Cell>> = Matrix::factory(self.size);

        for row in self.matrix.iter() {
            for cell in row.iter() {
                update_state(cell.point, &mut matrix_out);
            }
        }

        self.matrix = matrix_out;
    }
}
