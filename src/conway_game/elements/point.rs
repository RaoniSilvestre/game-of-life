#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new_default() -> Self {
        Point { row: 0, col: 0 }
    }
    pub fn new(x: usize, y: usize) -> Self {
        Point { row: x, col: y }
    }
}
