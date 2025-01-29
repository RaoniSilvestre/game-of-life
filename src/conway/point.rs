#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { row: x, col: y }
    }
}

impl From<&[u8]> for Point {
    fn from(bytes: &[u8]) -> Self {
        let row =
            usize::from_le_bytes(bytes[0..8].try_into().expect("Slice with incorrect lenght"));
        let col = usize::from_le_bytes(
            bytes[8..16]
                .try_into()
                .expect("Slice with incorrect lenght"),
        );
        Point { row, col }
    }
}

impl From<Point> for Vec<u8> {
    fn from(value: Point) -> Self {
        let mut bytes = Vec::with_capacity(16);

        bytes.extend_from_slice(&value.row.to_le_bytes());
        bytes.extend_from_slice(&value.col.to_le_bytes());

        bytes
    }
}
