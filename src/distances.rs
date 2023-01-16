use std::collections::HashMap;

pub struct Distances {
    pub root: (u8, u8),
    pub cells: std::collections::HashMap<(u8, u8), u32>,
}

impl Distances {
    pub fn new(row: u8, col: u8) -> Distances {
        let mut cells = HashMap::new();
        cells.insert((row, col), 0);
        Distances {
            root: (row, col),
            cells: cells,
        }
    }
}
