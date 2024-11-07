#[cfg(test)]
mod cell_tests {
    use gol::game::elements::{Cell, Point, State};
    use rand::Rng;

    #[test]
    fn should_create_the_default_cell() {
        let default_cell = Cell::new_default();
        let my_custom_cell = Cell::new(Point::new_default(), State::Dead);
        assert_eq!(default_cell, my_custom_cell)
    }

    #[test]
    fn should_assert_cell_state_and_point_for_each_iteration() {
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
}
