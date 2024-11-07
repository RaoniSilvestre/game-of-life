use game::elements::Point;
use rand::Rng;

// Funções auxiliares
pub fn random_generator(n: usize, size: Point) -> Vec<Point> {
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
