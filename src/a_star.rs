use crate::{cell::Cell, grid::Grid, position::Position};

pub fn tick(grid: &mut Grid) {
    let lowest = 0;
    // for i in 0..grid.open_set.len() {
    //     if grid.open_set[i] <
    // }

    grid.draw();
}
pub fn solve(grid: &mut Grid) {
    grid.open_set.push(grid.start);
    for row in &mut grid.grid {
        for cell in row {
            cell.set_h(&grid.target);
        }
    }
    while grid.open_set.len() > 0 {
        tick(grid);
    }
}

pub fn get_distance(cell1: &Position, cell2: &Position) -> i32 {
    let addent1 = (cell1.x as i32 - cell2.x as i32).abs();
    let addent2 = (cell1.y as i32 - cell2.y as i32).abs();
    return addent1 + addent2;
}
