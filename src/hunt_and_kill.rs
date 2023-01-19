use crate::grid::Grid;
use macroquad::rand::gen_range;

pub fn on(grid: &mut Grid) {
    let r = gen_range(0, grid.rows);
    let c = gen_range(0, grid.cols);

    let mut current = Some((grid.cells[r][c].row, grid.cells[r][c].col));

    while current.is_some() {
        let c = current.unwrap();
        // println!("{:?}", c);
        let all_neighbours = grid.grid_neighbours((c.0, c.1));

        let unvisited_neighbours: Vec<&(usize, usize)> = all_neighbours
            .iter()
            .filter(|(r, c)| !grid.cells[*r][*c].visited)
            .collect();

        // all_neighbours.iter().for_each(|&c| println!("{}, {}", c.0, c.1));
        // println!();

        grid.cells[c.0][c.1].visited = true;
        if unvisited_neighbours.len() > 0 {
            let idx = gen_range(0, unvisited_neighbours.len());
            current = Some((unvisited_neighbours[idx].0, unvisited_neighbours[idx].1));

            if c.0 == unvisited_neighbours[idx].0
                && c.1 as i32 == unvisited_neighbours[idx].1 as i32 - 1
            {
                grid.cells[c.0][c.1].remove_east_wall();
            } else if c.0 == unvisited_neighbours[idx].0 && c.1 == unvisited_neighbours[idx].1 + 1 {
                grid.cells[c.0][c.1].remove_west_wall();
            } else if c.0 == unvisited_neighbours[idx].0 + 1 && c.1 == unvisited_neighbours[idx].1 {
                grid.cells[c.0][c.1].remove_north_wall();
            } else if c.0 as i32 == unvisited_neighbours[idx].0 as i32 - 1
                && c.1 == unvisited_neighbours[idx].1
            {
                grid.cells[c.0][c.1].remove_south_wall();
            }
        } else {
            // println!("{:?}", current);
            current = None;
            for i in 0..grid.rows {
                let mut found = false;
                for j in 0..grid.cols {
                    if grid.cells[i][j].visited {
                        let all_neighbours = grid.grid_neighbours((i, j));
                        let unvisited_neighbours: Vec<&(usize, usize)> = all_neighbours
                            .iter()
                            .filter(|(r, c)| !grid.cells[*r][*c].visited)
                            .collect();
                        // all_neighbours.iter().for_each(|&c| println!("-- {}, {}", c.0, c.1));
                        if unvisited_neighbours.len() > 0 {
                            let idx = gen_range(0, unvisited_neighbours.len());
                            current =
                                Some((unvisited_neighbours[idx].0, unvisited_neighbours[idx].1));
                            found = true;

                            if i == unvisited_neighbours[idx].0
                                && j as i32 == unvisited_neighbours[idx].1 as i32 - 1
                            {
                                grid.cells[i][j].remove_east_wall();
                            } else if i == unvisited_neighbours[idx].0
                                && j == unvisited_neighbours[idx].1 + 1
                            {
                                grid.cells[i][j].remove_west_wall();
                            } else if i == unvisited_neighbours[idx].0 + 1
                                && j == unvisited_neighbours[idx].1
                            {
                                grid.cells[i][j].remove_north_wall();
                            } else if i as i32 == unvisited_neighbours[idx].0 as i32 - 1
                                && j == unvisited_neighbours[idx].1
                            {
                                grid.cells[i][j].remove_south_wall();
                            }
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
        }
    }
}
