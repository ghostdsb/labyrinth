use crate::grid::Grid;
use ::rand::*;

pub fn on(grid: &mut Grid) {
    let mut rng = thread_rng();
    for i in 0..grid.size {
        let mut run: Vec<(u8, u8)> = vec![];
        for j in 0..grid.size {
            let rnd = rng.gen_range(0..2);
            run.push((i, j));
            if i == grid.size - 1 && j == grid.size - 1 {
                continue;
            } else if rnd == 0 && j < grid.size - 1 || i == grid.size - 1 {
                grid.cells[i as usize][j as usize].remove_east_wall();
            } else {
                let rnd_index = rng.gen_range(0..run.len());
                let (r, c) = run[rnd_index];
                grid.cells[r as usize][c as usize].remove_south_wall();
                run.clear();
            }
        }
    }
}
