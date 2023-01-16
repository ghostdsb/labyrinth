use macroquad::prelude::*;
use crate::cell::Cell;

pub struct Grid{
    pub size: u8,
    pub cells: Vec<Vec<Cell>>,
}

const CELL_SIZE: f32 = 30.0;

impl Grid{
    pub fn new(size: u8)->Grid{
        let mut grid: Vec<Vec<Cell>> = vec!();
        for i in 0..size{
            let mut  r: Vec<Cell> = vec!();
            for j in 0..size{
                r.push(Cell::new(i,j));
            }
            grid.push(r);
        }
        Grid{
            size, cells: grid
        }
    }

    pub fn render(&self, algo: &str){
        // let mut c = 0;
        for i in 0..self.size{
            for j in 0..self.size{
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
                if cell.north_wall(){
                    draw_line(lt_x, lt_y, rt_x, rt_y, 1.0, BLUE);    
                }
            
                // west
                if cell.west_wall(){
                    draw_line(lt_x, lt_y, lb_x, lb_y, 1.0, BLUE);    
                }
            
                // south
                if cell.south_wall(){
                    draw_line(lb_x, lb_y, rb_x, rb_y, 1.0, BLUE);    
                }
            
                // east
                if cell.east_wall(){
                    draw_line(rt_x, rt_y, rb_x, rb_y, 1.0, BLUE);    
                }
                
                
            }
        }
        let algo = format!("- {}", algo);
        draw_text(&algo, CELL_SIZE*2.0, CELL_SIZE*12.0 - 5.0, 20.0, GREEN);
    
    }
}
