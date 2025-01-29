use super::{Neighbour, Point, State};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub state: State,
    pub point: Point,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            state: State::Dead,
            point: Point::default(),
        }
    }
}

impl Cell {
    pub fn new(point: Point, state: State) -> Self {
        Cell { state, point }
    }

    pub fn update_state_factory(
        matrix_in: &[Vec<Cell>],
        size: Point,
    ) -> impl Fn(Point, &mut [Vec<Cell>]) + '_ {
        move |point: Point, matrix_out: &mut [Vec<Cell>]| {
            Self::update_state(matrix_in, matrix_out, point, size)
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
        let alive_neighbours = Neighbour::alive_neighbours(matrix_in, point, size);
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

#[cfg(test)]
mod cell_tests {
    use rand::Rng;

    use crate::conway::{Cell, Point, State};

    #[test]
    fn should_create_the_default_cell() {
        let default_cell = Cell::default();
        let my_custom_cell = Cell::new(Point::default(), State::Dead);
        assert_eq!(default_cell, my_custom_cell)
    }

    #[test]
    fn should_assert_cell_state_and_point_for_each_iteration() {
        for _ in 0..200 {
            let mut rand = rand::rng();
            let rand_value_1 = rand.random_range(0..1000);
            let rand_value_2 = rand.random_range(0..1000);
            let mut rand_state = State::Dead;

            if rand_value_2 % 2 == 0 {
                rand_state = State::Alive;
            }

            let point = Point {
                row: rand_value_1,
                col: rand_value_2,
            };

            assert_eq!(
                Cell::new(point, rand_state),
                Cell {
                    point,
                    state: rand_state
                }
            )
        }
    }
}
