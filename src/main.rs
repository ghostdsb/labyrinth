use macroquad::prelude::*;
use std::env;

mod binary_tree;
mod cell;
mod constants;
mod distances;
mod grid;
mod side_winder;

const CELL_SIZE: f32 = constants::CELL_SIZE;
const GRID_SIZE: u8 = constants::GRID_SIZE;

#[macroquad::main(conf)]
async fn main() {
    let mut grid = grid::Grid::new(GRID_SIZE, grid::MODE::BACKGROUNDS);
    let arg = &env::args().nth(1).unwrap().parse::<u8>().unwrap();
    match arg {
        1 => {
            binary_tree::on(&mut grid);
        }
        2 => {
            side_winder::on(&mut grid);
        }
        _ => unimplemented!(),
    }

    configure(&mut grid);

    let start = (GRID_SIZE / 2, GRID_SIZE / 2);
    let target = (8, 8);

    distances::distances(&mut grid, start);
    distances::solution(&mut grid, start, target);

    let (max_distance, cell) = distances::farthest_cell(&grid, start);

    // let start = cell;
    // distances::distances(&mut grid, start);
    // let (max_distance, cell) = distances::farthest_cell(&grid, start);
    // distances::solution(&mut grid, start, cell);

    loop {
        grid.render(max_distance);
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
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Labyrinth"),
        window_width: CELL_SIZE as i32 * (GRID_SIZE + 2) as i32,
        window_height: CELL_SIZE as i32 * (GRID_SIZE + 2) as i32,
        ..Default::default()
    }
}
