use crate::cell::Cell;
use crate::constants;
use macroquad::prelude::*;

pub struct Grid {
    pub size: u8,
    pub cells: Vec<Vec<Cell>>,
}

const CELL_SIZE: f32 = constants::CELL_SIZE;

impl Grid {
    pub fn new(size: u8) -> Grid {
        let mut grid: Vec<Vec<Cell>> = vec![];
        for i in 0..size {
            let mut r: Vec<Cell> = vec![];
            for j in 0..size {
                r.push(Cell::new(i, j));
            }
            grid.push(r);
        }
        Grid { size, cells: grid }
    }

    pub fn neighbours(&self, index: (u8, u8)) -> Vec<(u8, u8)> {
        let mut neighbours = vec![];

        if let Some((r, c)) = self.cells[index.0 as usize][index.1 as usize].east_neighbour() {
            if !self.cells[index.0 as usize][index.1 as usize].east
                && !self.cells[r as usize][c as usize].west
            {
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0 as usize][index.1 as usize].west_neighbour() {
            if !self.cells[index.0 as usize][index.1 as usize].west
                && !self.cells[r as usize][c as usize].east
            {
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0 as usize][index.1 as usize].north_neighbour() {
            if !self.cells[index.0 as usize][index.1 as usize].north
                && !self.cells[r as usize][c as usize].south
            {
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0 as usize][index.1 as usize].south_neighbour() {
            if !self.cells[index.0 as usize][index.1 as usize].south
                && !self.cells[r as usize][c as usize].north
            {
                neighbours.push((r, c));
            }
        }

        neighbours
    }

    pub fn render(&self, algo: &str) {
        // let mut c = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                //  c += 1;
                let cell = &self.cells[i as usize][j as usize];
                let lt_y = cell.row as f32 * CELL_SIZE + CELL_SIZE;
                let lt_x = cell.col as f32 * CELL_SIZE + CELL_SIZE;

                let rt_x = lt_x + CELL_SIZE;
                let rt_y = lt_y;

                let lb_x = lt_x;
                let lb_y = lt_y + CELL_SIZE;

                let rb_x = rt_x;
                let rb_y = lb_y;

                // north
                if cell.north_wall() {
                    draw_line(lt_x, lt_y, rt_x, rt_y, 1.0, BLUE);
                }

                // west
                if cell.west_wall() {
                    draw_line(lt_x, lt_y, lb_x, lb_y, 1.0, BLUE);
                }

                // south
                if cell.south_wall() {
                    draw_line(lb_x, lb_y, rb_x, rb_y, 1.0, BLUE);
                }

                // east
                if cell.east_wall() {
                    draw_line(rt_x, rt_y, rb_x, rb_y, 1.0, BLUE);
                }

                let alpha = if cell.solution_path { 1.0 } else { 0.2 };
                // if cell.solution_path{
                let t = format!("{}", cell.distance);
                draw_text(
                    &t,
                    lt_x + CELL_SIZE / 2.0 - 5.0,
                    lt_y + CELL_SIZE / 2.0 + 5.0,
                    CELL_SIZE * 0.75,
                    Color {
                        r: 0.0,
                        g: 255.0,
                        b: 0.0,
                        a: alpha,
                    },
                );
                // }
            }
        }
        // let algo = format!("- {}", algo);
        // draw_text(&algo, CELL_SIZE * 2.0, CELL_SIZE * 12.0 - 5.0, 30.0, GREEN);
    }
}
