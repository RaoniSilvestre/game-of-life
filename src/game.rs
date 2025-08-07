use std::collections::HashSet;

use rand::seq::IndexedRandom;

pub type Point = (i32, i32);
pub type GameState = HashSet<Point>;

#[derive(Debug, Default)]
pub struct ConwayGame {
    alives: GameState,
    iterations: usize,
}

impl ConwayGame {
    pub fn add(&mut self, p: Point) {
        self.alives.insert(p);
    }

    pub fn remove(&mut self, p: Point) {
        self.alives.remove(&p);
    }

    pub fn state(&self) -> GameState {
        self.alives.clone()
    }

    pub fn add_around(&mut self, (a, b): Point) {
        let x_min = a - 4;
        let x_max = a + 3;
        let y_min = b - 4;
        let y_max = b + 3;

        let mut possible_points: Vec<Point> = Vec::with_capacity(64);
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                possible_points.push((x, y));
            }
        }

        let mut rng = rand::rng();
        let chosen_points = possible_points.as_slice().choose_multiple(&mut rng, 16);

        for point in chosen_points {
            self.add(*point);
        }
    }

    pub fn update(&mut self) {
        self.iterations += 1;

        let mut new_alives = HashSet::new();

        for (x, y) in self.alives.iter() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let point = (x + dx, y + dy);

                    let amount_of_neighbours = self.get_neighbours(point);
                    let is_alive = self.alives.contains(&point);

                    match (is_alive, amount_of_neighbours) {
                        (true, 2 | 3) => new_alives.insert(point),
                        (false, 3) => new_alives.insert(point),
                        _ => true,
                    };
                }
            }
        }

        self.alives = new_alives;
    }

    fn get_neighbours(&self, (x, y): Point) -> u8 {
        let mut alives = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let row = x + dx;
                let col = y + dy;

                if self.alives.contains(&(row, col)) {
                    alives += 1;
                }
            }
        }

        alives
    }
}
