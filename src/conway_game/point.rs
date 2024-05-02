#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub row: u16,
    pub col: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Point { row: x, col: y }
    }

    pub fn point_from_pointi32(point: PointI32) -> Result<Point, ()> {
        if point.row < 0 || point.col < 0 {
            return Err(());
        }

        Ok(Point {
            row: point.row as u16,
            col: point.col as u16,
        })
    }
}

#[derive(Clone, Copy)]
pub struct PointI32 {
    pub row: i32,
    pub col: i32,
}

impl PointI32 {
    pub fn new(x: i32, y: i32) -> Self {
        PointI32 { row: x, col: y }
    }
}

impl From<PointI32> for Point {
    // add code here
    fn from(value: PointI32) -> Self {
        Point {
            row: value.row as u16,
            col: value.col as u16,
        }
    }
}

impl From<Point> for PointI32 {
    // add code here
    fn from(value: Point) -> Self {
        PointI32 {
            row: value.row as i32,
            col: value.col as i32,
        }
    }
}
