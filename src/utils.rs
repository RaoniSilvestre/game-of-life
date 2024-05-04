use super::conway_game::elements::point::Point;
use rand::Rng;
use std::io::Stdout;
use terminal::*;

pub fn get_rand_points_list(n: usize, size: Point) -> Vec<Point> {
    let mut points_list = Vec::new();
    let mut rng_1 = rand::thread_rng();
    let mut rng_2 = rand::thread_rng();
    for _ in 0..n {
        let new_point = Point {
            row: rng_1.gen_range(1..size.row - 1),
            col: rng_2.gen_range(1..size.col - 1),
        };

        points_list.push(new_point)
    }
    points_list
}

pub fn get_ready_terminal() -> Terminal<Stdout> {
    let terminal = terminal::stdout();
    terminal.act(Action::HideCursor).unwrap();
    terminal.act(Action::ClearTerminal(Clear::All)).unwrap();

    terminal
}
