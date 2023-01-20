use crate::cell::Cell;
use crate::mask::Mask;
use crate::config;
use macroquad::prelude::*;

use image::{Rgb, RgbImage};
use imageproc::drawing::{
    draw_cross_mut, draw_filled_circle, draw_filled_circle_mut, draw_filled_rect_mut,
    draw_hollow_circle_mut, draw_hollow_rect_mut, draw_line_segment_mut,
};
use imageproc::rect::Rect;
use macroquad::rand::gen_range;
use std::path::Path;

#[derive(PartialEq)]
pub enum MODE {
    BACKGROUNDS,
    WALLS,
}

pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<Cell>>,
    pub path: Vec<Cell>,
}

const CELL_SIZE: f32 = config::CELL_SIZE;
const FLOW_COLOR: Color = config::FLOW_COLOR;
const FLOW_BACKGROUND: Color = config::FLOW_BACKGROUND;
const WALL_COLOR: Color = config::WALL_COLOR;
const TEXT_COLOR: Color = config::TEXT_COLOR;

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Grid {
        let mut grid: Vec<Vec<Cell>> = vec![];
        for i in 0..rows {
            let mut r: Vec<Cell> = vec![];
            for j in 0..cols {
                r.push(Cell::new(i, j, rows, cols));
            }
            grid.push(r);
        }
        Grid {
            cells: grid,
            rows,
            cols,
            path: vec![],
        }
    }

    pub fn apply_mask(&mut self, mask: Mask) {
        for i in 0..mask.bits.len(){
            for j in 0..mask.bits[i].len(){
                if mask.bits[i][j] == 0{
                    self.cells[i][j].is_alive = false;
                }
            }
        }
    }

    pub fn alive_cells(&self) -> u32{
        let mut c = 0;
        for i in 0..self.rows{
            for j in 0..self.cols{
                if self.cells[i][j].is_alive{
                    c += 1;
                }
            }
        }
        c
    }

    pub fn random_alive_cell(&self) -> (usize, usize) {
        let mut alive = vec!();
        for i in 0..self.rows{
            for j in 0..self.cols{
                if self.cells[i][j].is_alive{
                    alive.push((i,j));
                }
            }
        }
        let idx = gen_range(0, alive.len());

        alive[idx]
    }

    pub fn grid_neighbours(&self, index: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];

        if let Some((r, c)) = self.cells[index.0][index.1].east_neighbour() {
            if self.cells[r][c].is_alive{
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].west_neighbour() {
            if self.cells[r][c].is_alive{
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].north_neighbour() {
            if self.cells[r][c].is_alive{
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].south_neighbour() {
            if self.cells[r][c].is_alive{
                neighbours.push((r, c));
            }
        }

        neighbours
    }

    pub fn neighbours(&self, index: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];

        if let Some((r, c)) = self.cells[index.0][index.1].east_neighbour() {
            if !self.cells[index.0][index.1].east && !self.cells[r][c].west && self.cells[r][c].is_alive {
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].west_neighbour() {
            if !self.cells[index.0][index.1].west && !self.cells[r][c].east && self.cells[r][c].is_alive {
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].north_neighbour() {
            if !self.cells[index.0][index.1].north && !self.cells[r][c].south && self.cells[r][c].is_alive{
                neighbours.push((r, c));
            }
        }

        if let Some((r, c)) = self.cells[index.0][index.1].south_neighbour() {
            if !self.cells[index.0][index.1].south && !self.cells[r][c].north && self.cells[r][c].is_alive{
                neighbours.push((r, c));
            }
        }

        neighbours
    }

    pub fn save_to_image(&self, max_distance: u32, filename: &str, mode: MODE) {
        let path = Path::new(filename);
        let mut image = RgbImage::new(
            CELL_SIZE as u32 * (self.cols + 2) as u32,
            CELL_SIZE as u32 * (self.rows + 2) as u32,
        );

        for i in 0..self.rows {
            for j in 0..self.cols {
                let cell = &self.cells[i][j];
                let lt_y = cell.row as f32 * CELL_SIZE + CELL_SIZE;
                let lt_x = cell.col as f32 * CELL_SIZE + CELL_SIZE;

                let rt_x = lt_x + CELL_SIZE;
                let rt_y = lt_y;

                let lb_x = lt_x;
                let lb_y = lt_y + CELL_SIZE;

                let rb_x = rt_x;
                let rb_y = lb_y;

                let wall_color;

                if self.cells[i][j].is_alive {

                    if mode == MODE::BACKGROUNDS {
                        let mut col = FLOW_COLOR;
                        col.a = cell.distance as f32 / max_distance as f32;
                        wall_color = WALL_COLOR;
                        draw_filled_rect_mut(
                            &mut image,
                            Rect::at(lt_x as i32, lt_y as i32)
                                .of_size(CELL_SIZE as u32, CELL_SIZE as u32),
                            Rgb([
                                (wall_color.r * 255f32) as u8,
                                (wall_color.g * 255f32) as u8,
                                (wall_color.b * 255f32) as u8,
                            ]),
                        );
                        let intensity =
                            (max_distance as f32 - cell.distance as f32) / max_distance as f32;
                        let dark = intensity;
                        let bright = 0.5 + 0.5 * intensity;
                        draw_filled_rect_mut(
                            &mut image,
                            Rect::at(lt_x as i32, lt_y as i32)
                                .of_size(CELL_SIZE as u32, CELL_SIZE as u32),
                            Rgb([
                                (bright * 255f32) as u8,
                                (dark * 255f32) as u8,
                                (bright * 255f32) as u8,
                            ]),
                        );
                    } else {
                        draw_filled_rect_mut(
                            &mut image,
                            Rect::at(lt_x as i32, lt_y as i32)
                                .of_size(CELL_SIZE as u32, CELL_SIZE as u32),
                            Rgb([
                                (BLACK.r * 255f32) as u8,
                                (BLACK.g * 255f32) as u8,
                                (BLACK.b * 255f32) as u8,
                            ]),
                        );
                        wall_color = BLUE;
                    }
    
                    // north
                    if cell.north_wall() {
                        draw_line_segment_mut(
                            &mut image,
                            (lt_x as f32, lt_y as f32),
                            (rt_x as f32, rt_y as f32),
                            Rgb([
                                (wall_color.r * 255f32) as u8,
                                (wall_color.g * 255f32) as u8,
                                (wall_color.b * 255f32) as u8,
                            ]),
                        );
                    }
    
                    // west
                    if cell.west_wall() {
                        draw_line_segment_mut(
                            &mut image,
                            (lt_x as f32, lt_y as f32),
                            (lb_x as f32, lb_y as f32),
                            Rgb([
                                (wall_color.r * 255f32) as u8,
                                (wall_color.g * 255f32) as u8,
                                (wall_color.b * 255f32) as u8,
                            ]),
                        );
                    }
    
                    // south
                    if cell.south_wall() {
                        draw_line_segment_mut(
                            &mut image,
                            (lb_x as f32, lb_y as f32),
                            (rb_x as f32, rb_y as f32),
                            Rgb([
                                (wall_color.r * 255f32) as u8,
                                (wall_color.g * 255f32) as u8,
                                (wall_color.b * 255f32) as u8,
                            ]),
                        );
                    }
    
                    // east
                    if cell.east_wall() {
                        draw_line_segment_mut(
                            &mut image,
                            (rt_x as f32, rt_y as f32),
                            (rb_x as f32, rb_y as f32),
                            Rgb([
                                (wall_color.r * 255f32) as u8,
                                (wall_color.g * 255f32) as u8,
                                (wall_color.b * 255f32) as u8,
                            ]),
                        );
                    }

                }

            }
        }

        if mode == MODE::WALLS {
            // self.path
            //     .iter()
            //     .for_each(|c| println!("{},{}", c.row, c.col));
            for i in 0..self.path.len() - 1 {
                let s = (
                    self.path[i].col as f32 * CELL_SIZE + CELL_SIZE * 1.5,
                    self.path[i].row as f32 * CELL_SIZE + CELL_SIZE * 1.5,
                );
                let t = (
                    self.path[i + 1].col as f32 * CELL_SIZE + CELL_SIZE * 1.5,
                    self.path[i + 1].row as f32 * CELL_SIZE + CELL_SIZE * 1.5,
                );
                let col = GREEN;
                draw_filled_circle_mut(
                    &mut image,
                    (s.0 as i32, s.1 as i32),
                    1i32,
                    Rgb([
                        (col.r * 255f32) as u8,
                        (col.g * 255f32) as u8,
                        (col.b * 255f32) as u8,
                    ]),
                );
                draw_line_segment_mut(
                    &mut image,
                    (s.0, s.1),
                    (t.0, t.1),
                    Rgb([
                        (col.r * 255f32) as u8,
                        (col.g * 255f32) as u8,
                        (col.b * 255f32) as u8,
                    ]),
                );
            }
        }

        image.save(path).unwrap();
    }

    pub fn render(&self, max_distance: u32, mode: MODE) {
        draw_rectangle(
            CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE * (self.cols) as f32,
            CELL_SIZE * (self.cols) as f32,
            FLOW_BACKGROUND,
        );
        // let mut c = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
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

                if self.cells[i][j].is_alive {

                    if mode == MODE::BACKGROUNDS {
                        let mut col = FLOW_COLOR;
                        col.a = cell.distance as f32 / max_distance as f32;
                        wall_color = WALL_COLOR;
    
                        draw_rectangle(lt_x, lt_y, CELL_SIZE, CELL_SIZE, FLOW_BACKGROUND);
                        let intensity =
                            (max_distance as f32 - cell.distance as f32) / max_distance as f32;
                        let dark = intensity;
                        let bright = 0.5 + 0.5 * intensity;
                        draw_rectangle(
                            lt_x,
                            lt_y,
                            CELL_SIZE,
                            CELL_SIZE,
                            Color {
                                r: bright,
                                g: dark,
                                b: bright,
                                a: 1f32,
                            },
                        );
                    } else {
                        draw_rectangle(lt_x, lt_y, CELL_SIZE, CELL_SIZE, BLACK);
                        wall_color = BLUE;
                        let t = format!("{}", cell.distance);
                        draw_text(
                            &t,
                            lt_x + CELL_SIZE / 2.0 - 5.0,
                            lt_y + CELL_SIZE / 2.0 + 5.0,
                            CELL_SIZE * 0.6,
                            TEXT_COLOR,
                        );
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
}
