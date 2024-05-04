#[cfg(test)]
mod cell_tests {
    use cli_game_of_life::conway_game::elements::{
        cell::{Cell, State},
        matrix::CellMatrix,
        point::Point,
    };
    use rand::Rng;

    #[test]
    fn cell_new_test() {
        for _ in 0..200 {
            let mut rand = rand::thread_rng();
            let rand_value_1 = rand.gen_range(0..1000);
            let rand_value_2 = rand.gen_range(0..1000);
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

    #[test]
    fn alive_neighbors_test() {
        let mut matrix = CellMatrix::new_with_size(Point { row: 5, col: 5 });

        matrix[2][1].state = State::Alive;
        matrix[2][2].state = State::Alive;
        matrix[2][3].state = State::Alive;

        assert_eq!(matrix[2][2].alive_neighbours(&matrix), 2);
    }
}
