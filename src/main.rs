use macroquad::prelude::*;
use std::{collections::VecDeque, env};

mod binary_tree;
mod cell;
mod constants;
mod distances;
mod grid;
mod side_winder;

const CELL_SIZE: f32 = constants::CELL_SIZE;

#[macroquad::main(conf)]
async fn main() {
    let mut grid = grid::Grid::new(10);
    let mut distance = distances::Distances::new(0, 0);
    let arg = &env::args().nth(1).unwrap().parse::<u8>().unwrap();
    let algo: &str;
    match arg {
        1 => {
            binary_tree::on(&mut grid);
            algo = "Binary Tree";
        }
        2 => {
            side_winder::on(&mut grid);
            algo = "Side Winder";
        }
        _ => unimplemented!(),
    }

    configure(&mut grid);

    let mut queue: VecDeque<(u8, u8)> = VecDeque::new();
    queue.push_back((0, 0));
    while !queue.is_empty() {
        let (r, c) = queue.pop_front().unwrap();
        grid.cells[r as usize][c as usize].visited = true;
        grid.cells[r as usize][c as usize].distance += 1;
        let neighbours = grid.neighbours((r, c));
        for n in neighbours {
            if !grid.cells[n.0 as usize][n.1 as usize].visited {
                queue.push_back((n.0, n.1));
                grid.cells[n.0 as usize][n.1 as usize].distance =
                    grid.cells[r as usize][c as usize].distance;
            }
        }
    }

    let mut target = (9, 0);
    let mut breadcrumbs: Vec<(u8, u8)> = vec![];
    grid.cells[target.0 as usize][target.1 as usize].solution_path = true;
    while target != (0, 0) {
        let neighbours = grid.neighbours((target.0, target.1));
        for n in neighbours {
            if grid.cells[n.0 as usize][n.1 as usize].distance
                == grid.cells[target.0 as usize][target.1 as usize].distance - 1
            {
                grid.cells[n.0 as usize][n.1 as usize].solution_path = true;
                breadcrumbs.push((n.0, n.1));
                target = (n.0, n.1);
                break;
            }
        }
    }

    loop {
        grid.render(algo);
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
        window_width: CELL_SIZE as i32 * (10 + 2),
        window_height: CELL_SIZE as i32 * (10 + 2),
        ..Default::default()
    }
}
