use crate::grid::Grid;
use ::rand::*;

pub fn on(grid: &mut Grid) {
    let mut rng = thread_rng();
    for i in 0..grid.size {
        let mut run: Vec<(usize, usize)> = vec![];
        for j in 0..grid.size {
            let rnd = rng.gen_range(0..2);
            run.push((i, j));
            if i == grid.size - 1 && j == grid.size - 1 {
                continue;
            } else if rnd == 0 && j < grid.size - 1 || i == grid.size - 1 {
                grid.cells[i][j].remove_east_wall();
            } else {
                let rnd_index = rng.gen_range(0..run.len());
                let (r, c) = run[rnd_index];
                grid.cells[r][c].remove_south_wall();
                run.clear();
            }
        }
    }
}
