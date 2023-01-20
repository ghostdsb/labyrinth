use crate::grid::Grid;

pub fn scan(grid: &Grid)->(u32, u32){
    let mut dead_end_count = 0;
    let mut total_cells = 0;
    for i in 0..grid.rows{
        for j in 0..grid.cols{
            let neighbour_count = grid.neighbours((i,j)).len();
            if neighbour_count == 1{
                dead_end_count += 1;
            }
            if grid.cells[i][j].is_alive{
                total_cells += 1;
            }
        }
    }
    (dead_end_count, total_cells)
}