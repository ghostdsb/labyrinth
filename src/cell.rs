#[derive(PartialEq, Clone, Copy)]
pub struct Cell{
    pub row: u8,
    pub col: u8,
    pub east: bool,
    pub west: bool,
    pub north: bool,
    pub south: bool,
    pub visited: bool,
}

impl Cell{
    pub fn new(row: u8, col: u8) -> Cell{
        let (east, west, north, south) = (true, true, true, true);
        Cell { row, col, east, west, north, south, visited: false}
    }

    pub fn north_wall(&self) -> bool{
        self.north
    }
    pub fn east_wall(&self) -> bool{
        self.east
    }
    pub fn west_wall(&self) -> bool{
        self.west
    }
    pub fn south_wall(&self) -> bool{
        self.south
    }

    pub fn north_neighbour(&self) -> Option<(u8, u8)> {
        if self.row == 0 {
            None
        }else{
            Some((self.row-1, self.col))
        }
    }

    pub fn west_neighbour(&self) -> Option<(u8, u8)> {
        if self.col == 0 {
            None
        }else{
            Some((self.row, self.col-1))
        }
    }

    pub fn remove_east_wall(&mut self){
        self.east = false
    }
    
    pub fn remove_south_wall(&mut self){
        self.south = false
    }

}
