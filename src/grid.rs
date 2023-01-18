use crate::cell::Cell;
use crate::{config, GRID_SIZE};
use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum MODE {
    BACKGROUNDS,
    WALLS,
}

pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
    mode: MODE,
}

const CELL_SIZE: f32 = config::CELL_SIZE;
const FLOW_COLOR: Color = config::FLOW_COLOR;
const FLOW_BACKGROUND: Color = config::FLOW_BACKGROUND;
const WALL_COLOR: Color = config::WALL_COLOR;
const TEXT_COLOR: Color = config::TEXT_COLOR;

impl Grid {
    pub fn new(size: usize, mode: MODE) -> Grid {
        let mut grid: Vec<Vec<Cell>> = vec![];
        for i in 0..size {
            let mut r: Vec<Cell> = vec![];
            for j in 0..size {
                r.push(Cell::new(i, j));
            }
            grid.push(r);
        }
        Grid {
            size,
            cells: grid,
            mode,
        }
    }

    pub fn grid_neighbours(&self, index: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];

        if let Some((r, c)) = self.cells[index.0][index.1].east_neighbour() {
            neighbours.push((r, c));
        }

        if let Some((r, c)) = self.cells[index.0][index.1].west_neighbour() {
            neighbours.push((r, c));
        }

        if let Some((r, c)) = self.cells[index.0][index.1].north_neighbour() {
            neighbours.push((r, c));
        }

        if let Some((r, c)) = self.cells[index.0][index.1].south_neighbour() {
            neighbours.push((r, c));
        }

        neighbours
    }

    pub fn neighbours(&self, index: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];

        if let Some((r, c)) = self.cells[index.0][index.1].east_neighbour() {
            if !self.cells[index.0][index.1].east && !self.cells[r][c].west {
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].west_neighbour() {
            if !self.cells[index.0][index.1].west && !self.cells[r][c].east {
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].north_neighbour() {
            if !self.cells[index.0][index.1].north && !self.cells[r][c].south {
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].south_neighbour() {
            if !self.cells[index.0][index.1].south && !self.cells[r][c].north {
                neighbours.push((r, c));
            }
        }

        neighbours
    }

    pub fn render(&self, max_distance: u32) {
        draw_rectangle(CELL_SIZE, CELL_SIZE, CELL_SIZE*(GRID_SIZE) as f32, CELL_SIZE* (GRID_SIZE) as f32, ORANGE);
        // let mut c = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                //  c += 1;
                let cell = &self.cells[i][j];
                let lt_y = cell.row as f32 * CELL_SIZE + CELL_SIZE;
                let lt_x = cell.col as f32 * CELL_SIZE + CELL_SIZE;

                let rt_x = lt_x + CELL_SIZE;
                let rt_y = lt_y;

                let lb_x = lt_x;
                let lb_y = lt_y + CELL_SIZE;

                let rb_x = rt_x;
                let rb_y = lb_y;

                let mut wall_color = WALL_COLOR;

                if self.mode == MODE::BACKGROUNDS {
                    let mut col = FLOW_COLOR;
                    col.a = cell.distance as f32 / max_distance as f32;
                    wall_color = WALL_COLOR;
                    draw_rectangle(lt_x, lt_y, CELL_SIZE, CELL_SIZE, FLOW_BACKGROUND);
                    draw_rectangle(lt_x, lt_y, CELL_SIZE, CELL_SIZE, col);
                }else {
                    // draw_rectangle(lt_x, lt_y, CELL_SIZE*(GRID_SIZE-10) as f32, CELL_SIZE* (GRID_SIZE-10) as f32, ORANGE);
                    // let a = if cell.solution_path { 1.0 } else { 0.5 };
                    if cell.solution_path {
                        let mut col = FLOW_COLOR;
                        col.a = cell.distance as f32 / max_distance as f32;
                        wall_color = WALL_COLOR;
                        draw_rectangle(lt_x, lt_y, CELL_SIZE, CELL_SIZE, FLOW_BACKGROUND);
                        draw_rectangle(lt_x, lt_y, CELL_SIZE, CELL_SIZE, col);
                        let t = format!("{}", cell.distance);
                        // let t = format!("{}, {}", cell.row, cell.col);
                        draw_text(
                            &t,
                            lt_x + CELL_SIZE / 2.0 - 5.0/3.0,
                            lt_y + CELL_SIZE / 2.0 + 5.0/3.0,
                            8.0,
                            // CELL_SIZE * 0.75,
                            TEXT_COLOR,
                        );
                    }
                }

                // north
                if cell.north_wall() {
                    draw_line(lt_x, lt_y, rt_x, rt_y, 1.0, wall_color);
                }

                // west
                if cell.west_wall() {
                    draw_line(lt_x, lt_y, lb_x, lb_y, 1.0, wall_color);
                }

                // south
                if cell.south_wall() {
                    draw_line(lb_x, lb_y, rb_x, rb_y, 1.0, wall_color);
                }

                // east
                if cell.east_wall() {
                    draw_line(rt_x, rt_y, rb_x, rb_y, 1.0, wall_color);
                }

            }
        }
    }
}
