use crate::grid::Grid;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn on(grid: &mut Grid) {
    let mut unvisited: Vec<(usize, usize)> = vec![];
    grid.cells.iter().for_each(|row| {
        row.iter()
            .for_each(|cell| unvisited.push((cell.row, cell.col)))
    });

    // unvisited.shuffle(&mut thread_rng());
    unvisited.pop();

    while !unvisited.is_empty() {
        unvisited.shuffle(&mut thread_rng());
        let mut cell = *unvisited.iter().nth(0).unwrap();
        let mut path: Vec<(usize, usize)> = vec![cell];
        while unvisited
            .iter_mut()
            .any(|&mut c| c.0 == cell.0 && c.1 == cell.1)
        {
            let mut neighbours = grid.grid_neighbours(cell);
            neighbours.shuffle(&mut thread_rng());
            cell = neighbours.pop().unwrap();
            while path.iter_mut().any(|c| c.0 == cell.0 && c.1 == cell.1) {
                path.pop();
            }
            path.push(cell);
            // path.iter().for_each(|c| println!("{:?}", c));
        }
        // println!();
        for i in 0..path.len() - 1 {
            // grid.cells[path[i].0][path[i].1].visited = true;

            if path[i + 1].0 == path[i].0 && path[i + 1].1 as i32 == path[i].1 as i32 - 1 {
                grid.cells[path[i].0][path[i].1].remove_west_wall();
            } else if path[i + 1].0 == path[i].0 && path[i + 1].1 == path[i].1 + 1 {
                grid.cells[path[i].0][path[i].1].remove_east_wall();
            } else if path[i + 1].0 as i32 == path[i].0 as i32 - 1 && path[i + 1].1 == path[i].1 {
                grid.cells[path[i].0][path[i].1].remove_north_wall();
            } else if path[i + 1].0 == path[i].0 + 1 && path[i + 1].1 == path[i].1 {
                grid.cells[path[i].0][path[i].1].remove_south_wall();
            }
            let (idx, _) = unvisited
                .iter()
                .enumerate()
                .find(|(_id, (r, c))| *r == path[i].0 && *c == path[i].1)
                .unwrap();
            unvisited.remove(idx);
        }
    }
}
