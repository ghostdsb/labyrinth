use grid::MODE;
use macroquad::prelude::*;
use std::env;

mod aldous_broder;
mod binary_tree;
mod cell;
mod config;
mod distances;
mod grid;
mod hunt_and_kill;
mod recursive_backtracker;
mod side_winder;
mod wilson;
mod mask;
mod image_scanner;

const CELL_SIZE: f32 = config::CELL_SIZE;
const GRID_SIZE: usize = config::GRID_SIZE;
const MODE: MODE = config::MODE;

#[macroquad::main(conf)]
async fn main() {

    // image_scanner::create_mask("repo.png");
    let mask_data = mask::Mask::new("rp.png");

    let mut grid = grid::Grid::new(mask_data.row,mask_data.col);

    grid.apply_mask(mask_data);

    let arg = &env::args().nth(1).unwrap().parse::<usize>().unwrap();
    
    let mut algorithm = "algo";
    match arg {
        1 => {
            binary_tree::on(&mut grid);
            algorithm = "binary-tree";
        }
        2 => {
            side_winder::on(&mut grid);
            algorithm = "sidewinder";
        }
        3 => {
            aldous_broder::on(&mut grid);
            algorithm = "aldous-broder";
        }
        4 => {
            wilson::on(&mut grid);
            algorithm = "wilson";
        }
        5 => {
            hunt_and_kill::on(&mut grid);
            algorithm = "hunt-and-kill";
        }
        6 => {
            recursive_backtracker::on(&mut grid);
            algorithm = "recursive-backtracker";
        }
        7 => {
            recursive_backtracker::on(&mut grid);
            algorithm = "recursive-backtracker-scanner";
        }
        _ => unimplemented!(),
    }

    configure(&mut grid);

    let start = grid.random_alive_cell();
    distances::distances(&mut grid, start);

    let (_max_distance, cell) = distances::farthest_cell(&grid, start);

    let start = cell;
    distances::distances(&mut grid, start);
    let (max_distance, cell) = distances::farthest_cell(&grid, start);
    distances::solution(&mut grid, start, cell);

    grid.save_to_image(
        max_distance,
        &format!("img/{}-bg.png", algorithm),
        MODE::BACKGROUNDS,
    );
    grid.save_to_image(
        max_distance,
        &format!("img/{}-path.png", algorithm),
        MODE::WALLS,
    );

    loop {
        grid.render((max_distance as f32 * 0.8) as u32, MODE::BACKGROUNDS);
        next_frame().await
    }
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
