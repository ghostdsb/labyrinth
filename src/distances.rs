use crate::grid::Grid;
use std::collections::VecDeque;

pub fn distances(grid: &mut Grid, start: (usize, usize)) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            grid.cells[i][j].distance = 0;
            grid.cells[i][j].visited = false;
        }
    }

    queue.push_back(start);
    while !queue.is_empty() {
        let (r, c) = queue.pop_front().unwrap();
        grid.cells[r][c].visited = true;
        grid.cells[r][c].distance += 1;
        let neighbours = grid.neighbours((r, c));
        for n in neighbours {
            if !grid.cells[n.0][n.1].visited {
                queue.push_back((n.0, n.1));
                grid.cells[n.0][n.1].distance = grid.cells[r][c].distance;
            }
        }
    }
}

pub fn solution(grid: &mut Grid, start: (usize, usize), end: (usize, usize)) {
    let mut breadcrumbs: Vec<(usize, usize)> = vec![];
    let mut target = end;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            grid.cells[i][j].solution_path = false;
        }
    }
    grid.cells[target.0][target.1].solution_path = true;
    grid.path.push(grid.cells[target.0][target.1]);
    while target != start {
        let neighbours = grid.neighbours((target.0, target.1));
        for n in neighbours {
            if grid.cells[n.0][n.1].distance as i32
                == grid.cells[target.0][target.1].distance as i32 - 1
            {
                grid.cells[n.0][n.1].solution_path = true;
                breadcrumbs.push((n.0, n.1));
                target = (n.0, n.1);
                grid.path.push(grid.cells[target.0][target.1]);
                break;
            }
        }
    }
}

pub fn farthest_cell(grid: &Grid, start: (usize, usize)) -> (u32, (usize, usize)) {
    let mut max = 1;
    let mut cell = start;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid.cells[i][j].distance > max {
                max = grid.cells[i][j].distance;
                cell = (i, j);
            }
        }
    }
    (max, cell)
}
