use grid::MODE;
use macroquad::prelude::*;
use std::env;

mod aldous_broder;
mod binary_tree;
mod cell;
mod config;
mod distances;
mod grid;
mod side_winder;
mod wilson;

const CELL_SIZE: f32 = config::CELL_SIZE;
const GRID_SIZE: usize = config::GRID_SIZE;
const MODE: MODE = config::MODE;

#[macroquad::main(conf)]
async fn main() {
    let mut grid = grid::Grid::new(GRID_SIZE, MODE);
    let arg = &env::args().nth(1).unwrap().parse::<usize>().unwrap();
    match arg {
        1 => {
            binary_tree::on(&mut grid);
        }
        2 => {
            side_winder::on(&mut grid);
        }
        3 => {
            aldous_broder::on(&mut grid);
        }
        4 => {
            wilson::on(&mut grid);
        }
        _ => unimplemented!(),
    }

    configure(&mut grid);

    let start = (GRID_SIZE / 2, GRID_SIZE / 2);
    let target = (GRID_SIZE - 1, GRID_SIZE - 1);

    distances::distances(&mut grid, start);
    distances::solution(&mut grid, start, target);

    let (max_distance, cell) = distances::farthest_cell(&grid, start);

    // let start = cell;
    // distances::distances(&mut grid, start);
    // let (max_distance, cell) = distances::farthest_cell(&grid, start);
    // distances::solution(&mut grid, start, cell);

    loop {
        grid.render(25);
        next_frame().await
    }
}

fn configure(grid: &mut grid::Grid) {
    for i in 0..grid.size {
        for j in 0..grid.size {
            if let Some((r, c)) = grid.cells[i as usize][j as usize].north_neighbour() {
                if !grid.cells[r as usize][c as usize].south {
                    grid.cells[i as usize][j as usize].north = false;
                }
            }

            if let Some((r, c)) = grid.cells[i as usize][j as usize].west_neighbour() {
                if !grid.cells[r as usize][c as usize].east {
                    grid.cells[i as usize][j as usize].west = false;
                }
            }

            if let Some((r, c)) = grid.cells[i as usize][j as usize].east_neighbour() {
                if !grid.cells[r as usize][c as usize].west {
                    grid.cells[i as usize][j as usize].east = false;
                }
            }

            if let Some((r, c)) = grid.cells[i as usize][j as usize].south_neighbour() {
                if !grid.cells[r as usize][c as usize].north {
                    grid.cells[i as usize][j as usize].south = false;
                }
            }
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Labyrinth"),
        window_width: CELL_SIZE as i32 * (GRID_SIZE + 2) as i32,
        window_height: CELL_SIZE as i32 * (GRID_SIZE + 2) as i32,
        sample_count: 1,
        ..Default::default()
    }
}
