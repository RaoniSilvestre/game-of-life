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

    pub fn update_state_factory(
        matrix_in: &[Vec<Cell>],
        size: Point,
    ) -> impl Fn(Point, &mut [Vec<Cell>]) + '_ {
        move |point: Point, matrix_out: &mut [Vec<Cell>]| {
            Self::update_state(&matrix_in, matrix_out, point, size)
        }
    }

    fn update_state(
        matrix_in: &[Vec<Cell>],
        matrix_out: &mut [Vec<Cell>],
        point: Point,
        size: Point,
    ) {
        let x = point.row % size.row;
        let y = point.col % size.col;

        let actual_cell = matrix_in[x][y];
        let alive_neighbours = Neighbour::alive_neighbours(&matrix_in, point, size);
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
