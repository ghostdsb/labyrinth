use crate::grid::Grid;
use ::rand::*;

pub fn on(grid: &mut Grid) {
    let mut rng = thread_rng();

    grid.cells.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|cell| {
            let rnd = rng.gen_range(0..2);
            if cell.row == grid.size - 1 && cell.col == grid.size - 1 {
                ();
            } else if cell.row == grid.size - 1 {
                cell.remove_east_wall();
            } else if cell.col == grid.size - 1 {
                cell.remove_south_wall();
            } else if rnd == 0 {
                cell.remove_east_wall();
            } else {
                cell.remove_south_wall();
            }
        })
    });
}
