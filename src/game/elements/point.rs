use crate::game::elements::Point;

impl Point {
    pub fn new_default() -> Self {
        Point { row: 0, col: 0 }
    }
    pub fn new(x: usize, y: usize) -> Self {
        Point { row: x, col: y }
    }
}

impl From<&[u8]> for Point {
    // add code here
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

impl Into<Vec<u8>> for Point {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&self.row.to_le_bytes());
        bytes.extend_from_slice(&self.col.to_le_bytes());
        bytes
    }
}
