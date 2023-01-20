use crate::grid::Grid;
use macroquad::rand::gen_range;

pub fn on(grid: &mut Grid) {
    let (r, c) = (5,5);
    // let (r, c) = grid.random_alive_cell();

    let mut current = (grid.cells[r][c].row, grid.cells[r][c].col);

    let mut stack = vec![current];

    while !stack.is_empty() {
        current = *stack.last().unwrap();

        let all_neighbours = grid.grid_neighbours((current.0, current.1));

        let unvisited_neighbours: Vec<&(usize, usize)> = all_neighbours
            .iter()
            .filter(|(r, c)| !grid.cells[*r][*c].visited)
            .collect();

        let i = current.0;
        let j = current.1;
        grid.cells[i][j].visited = true;
        if unvisited_neighbours.is_empty() {
            stack.pop();
        } else {
            let idx = gen_range(0, unvisited_neighbours.len());
            if i == unvisited_neighbours[idx].0
                && j as i32 == unvisited_neighbours[idx].1 as i32 - 1
            {
                grid.cells[i][j].remove_east_wall();
            } else if i == unvisited_neighbours[idx].0 && j == unvisited_neighbours[idx].1 + 1 {
                grid.cells[i][j].remove_west_wall();
            } else if i == unvisited_neighbours[idx].0 + 1 && j == unvisited_neighbours[idx].1 {
                grid.cells[i][j].remove_north_wall();
            } else if i as i32 == unvisited_neighbours[idx].0 as i32 - 1
                && j == unvisited_neighbours[idx].1
            {
                grid.cells[i][j].remove_south_wall();
            }
            stack.push((unvisited_neighbours[idx].0, unvisited_neighbours[idx].1));
        }
    }
}
