use crate::grid::Grid;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn on(grid: &mut Grid) {
    let mut current = (0, 0);
    let mut count = 0;
    while count != grid.size * grid.size - 1 {
        grid.cells[current.0][current.1].visited = true;

        let mut neighbours = grid.grid_neighbours(current);
        neighbours.shuffle(&mut thread_rng());

        if let Some((r, c)) = neighbours.iter().nth(0) {
            if !grid.cells[*r][*c].visited {
                count += 1;
                if current.0 == *r && current.1 as i8 == *c as i8 - 1 {
                    grid.cells[current.0][current.1].remove_east_wall();
                } else if current.0 == *r && current.1 == *c + 1 {
                    grid.cells[current.0][current.1].remove_west_wall();
                } else if current.0 as i8 == *r as i8 - 1 && current.1 == *c {
                    grid.cells[current.0][current.1].remove_south_wall();
                } else {
                    grid.cells[current.0][current.1].remove_north_wall();
                }
            }
            current = (*r, *c);
        };
        // println!("{}", count);
    }
}
