use macroquad::prelude::*;
use std::env;

mod cell;
mod grid;
mod binary_tree;
mod side_winder;

#[macroquad::main(conf)]
async fn main() {
    let mut grid = grid::Grid::new(10);
    let arg = &env::args().nth(1).unwrap().parse::<u8>().unwrap();
    let algo: &str;
    match arg{
        1 => {
            binary_tree::on(&mut grid); 
            algo = "Binary Tree";
        },
        2 => {side_winder::on(&mut grid);
            algo = "Side Winder";
        },
        _ => unimplemented!()
    }
    configure(&mut grid);
    loop {
        grid.render(algo);    
        next_frame().await
    }
}

fn configure(grid: &mut grid::Grid){

    for i in 0..grid.size{
        for j in 0..grid.size{
            if let Some((r,c)) = grid.cells[i as usize][j as usize].north_neighbour(){
                if !grid.cells[r as usize][c as usize].south{
                    grid.cells[i as usize][j as usize].north = false;
                }
            }

            if let Some((r,c)) = grid.cells[i as usize][j as usize].west_neighbour(){
                if !grid.cells[r as usize][c as usize].east{
                    grid.cells[i as usize][j as usize].west = false;
                }
            }
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Labyrinth"),
        window_width: 360,
        window_height: 360,
        ..Default::default()
    }
}