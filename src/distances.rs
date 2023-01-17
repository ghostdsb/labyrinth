use crate::grid::Grid;
use std::collections::VecDeque;

pub fn distances(grid: &mut Grid, start: (u8, u8)) {
    let mut queue: VecDeque<(u8, u8)> = VecDeque::new();

    for i in 0..grid.size {
        for j in 0..grid.size {
            grid.cells[i as usize][j as usize].distance = 0;
            grid.cells[i as usize][j as usize].visited = false;
        }
    }

    queue.push_back(start);
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

    // let mut breadcrumbs: Vec<(u8, u8)> = vec![];
    // grid.cells[target.0 as usize][target.1 as usize].solution_path = true;
    // while target != start {
    //     let neighbours = grid.neighbours((target.0, target.1));
    //     for n in neighbours {
    //         if grid.cells[n.0 as usize][n.1 as usize].distance
    //             == grid.cells[target.0 as usize][target.1 as usize].distance - 1
    //         {
    //             grid.cells[n.0 as usize][n.1 as usize].solution_path = true;
    //             breadcrumbs.push((n.0, n.1));
    //             target = (n.0, n.1);
    //             break;
    //         }
    //     }
    // }
}

pub fn solution(grid: &mut Grid, start: (u8, u8), end: (u8, u8)) {
    let mut breadcrumbs: Vec<(u8, u8)> = vec![];
    let mut target = end;
    for i in 0..grid.size {
        for j in 0..grid.size {
            grid.cells[i as usize][j as usize].solution_path = false;
        }
    }
    grid.cells[target.0 as usize][target.1 as usize].solution_path = true;
    while target != start {
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
}

pub fn farthest_cell(grid: &Grid, start: (u8, u8)) -> (u32, (u8, u8)) {
    let mut max = 1;
    let mut cell = start;
    for i in 0..grid.size {
        for j in 0..grid.size {
            if grid.cells[i as usize][j as usize].distance > max {
                max = grid.cells[i as usize][j as usize].distance;
                cell = (i, j);
            }
        }
    }
    (max, cell)
}
