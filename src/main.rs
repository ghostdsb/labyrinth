use grid::{MODE, Grid};
use macroquad::prelude::*;
use std::env;

mod aldous_broder;
mod binary_tree;
mod cell;
mod config;
mod distances;
mod grid;
mod hunt_and_kill;
mod mask;
mod recursive_backtracker;
mod side_winder;
mod wilson;

mod deadend;

const CELL_SIZE: f32 = config::CELL_SIZE;
const GRID_SIZE: usize = config::GRID_SIZE;

#[macroquad::main(conf)]
async fn main() {
    let arg = &env::args().nth(1).unwrap().parse::<usize>().unwrap();

    match arg{
        0 => deadend_analytics(),
        1..=9 => {
            let (grid, max_distance) = builder(*arg, true);
            
            loop {
                grid.render((max_distance as f32 * 0.8) as u32, MODE::BACKGROUNDS);
                next_frame().await
            }
        }
        _ => unimplemented!()
    }
    
}

#[allow(dead_code)]
fn deadend_analytics(){
    for i in 1..=6{
        let (grid, max_distance) = builder(i, false);
        let (deadend, total_cells) = deadend::scan(&grid);
        println!("{}-> longest_path_length: {}, deadend: {}/{}, %: {}", i, max_distance, deadend, total_cells, deadend*100/total_cells);
    }
}

fn builder<'a>(algo: usize, save_image: bool) -> (Grid, u32){
    let algorithm;
    let mut grid: grid::Grid;
    let mut start: (usize, usize) = (0,0);

    match algo {
        1 => {
            grid = grid::Grid::new(20, 20);
            binary_tree::on(&mut grid);
            algorithm = "binary-tree";
        }
        2 => {
            grid = grid::Grid::new(20, 20);
            side_winder::on(&mut grid);
            algorithm = "sidewinder";
        }
        3 => {
            grid = grid::Grid::new(20, 20);
            aldous_broder::on(&mut grid);
            algorithm = "aldous-broder";
        }
        4 => {
            grid = grid::Grid::new(20, 20);
            wilson::on(&mut grid);
            algorithm = "wilson";
        }
        5 => {
            grid = grid::Grid::new(20, 20);
            hunt_and_kill::on(&mut grid);
            algorithm = "hunt-and-kill";
        }
        6 => {
            grid = grid::Grid::new(20, 20);
            recursive_backtracker::on(&mut grid);
            algorithm = "recursive-backtracker";
        }
        7 => {
            let mask_data = mask::Mask::new("masks/aapa.png", mask::MaskType::Image);
            grid = grid::Grid::new(mask_data.row, mask_data.col);
            grid.apply_mask(mask_data);
            start = grid.random_alive_cell();
            recursive_backtracker::on(&mut grid);
            algorithm = "recursive-backtracker-image-scanner";
        }
        8 => {
            let mask_data = mask::Mask::new("masks/mask.txt", mask::MaskType::Text);
            println!("{:?}", mask_data.bits);
            grid = grid::Grid::new(mask_data.row, mask_data.col);
            grid.apply_mask(mask_data);
            start = grid.random_alive_cell();
            recursive_backtracker::on(&mut grid);
            algorithm = "recursive-backtracker-text-scanner";
        }
        9 => {
            let mask_data = mask::Mask::new("masks/pan.png", mask::MaskType::Image);
            grid = grid::Grid::new(mask_data.row, mask_data.col);
            grid.apply_mask(mask_data);
            start = grid.random_alive_cell();
            recursive_backtracker::on(&mut grid);
            algorithm = "recursive-backtracker-image-scanner-cover";
        }
        _ => unimplemented!(),
    }

    configure(&mut grid);

    distances::distances(&mut grid, start);
    let (_max_distance, cell) = distances::farthest_cell(&grid, start);
    start = cell;
    distances::distances(&mut grid, start);
    let (max_distance, cell) = distances::farthest_cell(&grid, start);
    distances::solution(&mut grid, start, cell);

    if save_image {
        (0..8).for_each(|id|{
            grid.save_to_image(
                max_distance,
                &format!("choice/{}-{}-bg.png", id, algorithm),
                MODE::BACKGROUNDS,
                false,
                id
            );
        });
        grid.save_to_image(
            max_distance,
            &format!("choice/{}.png", algorithm),
            MODE::WALLS,
            false,
            0
        );
        grid.save_to_image(
            max_distance,
            &format!("choice/{}-solved.png", algorithm),
            MODE::WALLS,
            true,
            0
        );
    }

    (grid, max_distance)
}

fn configure(grid: &mut grid::Grid) {
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if let Some((r, c)) = grid.cells[i][j].north_neighbour() {
                if !grid.cells[r][c].south {
                    grid.cells[i][j].north = false;
                }
            }

            if let Some((r, c)) = grid.cells[i][j].west_neighbour() {
                if !grid.cells[r][c].east {
                    grid.cells[i][j].west = false;
                }
            }

            if let Some((r, c)) = grid.cells[i][j].east_neighbour() {
                if !grid.cells[r][c].west {
                    grid.cells[i][j].east = false;
                }
            }

            if let Some((r, c)) = grid.cells[i][j].south_neighbour() {
                if !grid.cells[r][c].north {
                    grid.cells[i][j].south = false;
                }
            }
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Labyrinth"),
        window_width: CELL_SIZE as i32 * (GRID_SIZE + 2 + 10) as i32,
        window_height: CELL_SIZE as i32 * (GRID_SIZE + 2 + 10) as i32,
        sample_count: 1,
        ..Default::default()
    }
}
