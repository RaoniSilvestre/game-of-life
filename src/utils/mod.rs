use rand::Rng;

use crate::conway::Point;

pub fn random_generator(n: usize, size: Point) -> Vec<Point> {
    let mut points_list = Vec::new();
    let mut rng_1 = rand::rng();
    let mut rng_2 = rand::rng();
    for _ in 0..n {
        let new_point = Point {
            row: rng_1.random_range(1..size.row - 1),
            col: rng_2.random_range(1..size.col - 1),
        };

        points_list.push(new_point)
    }
    points_list
}
