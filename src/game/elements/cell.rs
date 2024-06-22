use super::{Cell, Neighbour, Point, State};

impl Cell {
    // Inicializar celula
    pub fn new_default() -> Self {
        Cell {
            state: State::Dead,
            point: Point::new_default(),
        }
    }

    pub fn new(point: Point, state: State) -> Self {
        Cell { state, point }
    }

    pub fn update_state_factory(matrix_in: &[Vec<Cell>]) -> impl Fn(Point, &mut [Vec<Cell>]) + '_ {
        move |point: Point, matrix_out: &mut [Vec<Cell>]| {
            Self::update_state(&matrix_in, point, matrix_out)
        }
    }

    fn update_state(matrix_in: &[Vec<Cell>], point: Point, matrix_out: &mut [Vec<Cell>]) {
        let actual_cell = matrix_in[point.row][point.col];

        let alive_neighbours = Neighbour::alive_neighbours(point, &matrix_in);

        let new_state = match actual_cell.state {
            State::Alive => {
                if alive_neighbours == 2 || alive_neighbours == 3 {
                    State::Alive
                } else {
                    State::Dead
                }
            }
            State::Dead => {
                if alive_neighbours == 3 {
                    State::Alive
                } else {
                    State::Dead
                }
            }
        };

        matrix_out[point.row][point.col].state = new_state;
    }
}
