use crate::color::Color;
use crate::grid::Stage;
use crate::{grid::Grid, position::Position};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use Color::{ClosedSet, OpenSet, PATH};
// use color::CLOSED_SET;
pub fn tick(grid: &mut Grid) {
    grid.draw();
    if grid.open_set.len() < 1 {
        grid.stage = Stage::Done;
        return;
    }
    let mut lowest = 0;
    let mut lowest_pos = grid.open_set[lowest];
    for i in 0..grid.open_set.len() {
        let item = grid.open_set[i];
        if grid.grid[item.x][item.y].f() < grid.grid[lowest_pos.x][lowest_pos.y].f() {
            lowest = i;
            lowest_pos = grid.open_set[lowest];
        } else if grid.compare_h
            && grid.grid[item.x][item.y].f() == grid.grid[lowest_pos.x][lowest_pos.y].f()
        {
            // TODO implement toggle for comparing h scores
            if grid.grid[item.x][item.y].h < grid.grid[lowest_pos.x][lowest_pos.y].h {
                lowest = i;
                lowest_pos = grid.open_set[lowest];
            }
        }
    }

    if lowest_pos.x == grid.target.x && lowest_pos.y == grid.target.y {
        grid.next_to_show = Some(grid.target);
        grid.stage = Stage::DrawingPath;
        return;
    };
    let current = grid.open_set.remove(lowest);
    grid.closed_set.push(current);

    if current.x != grid.start.x || current.y != grid.start.y {
        grid.grid[current.x][current.y].color = Color::get(ClosedSet);
    }
    let current_neighbors = grid.grid[current.x][current.y].get_neighbors(grid);
    for n in current_neighbors {
        if !(grid.closed_set.contains(&n)) {
            let temp_g = grid.grid[current.x][current.y].g + get_distance(&current, &n);
            if grid.open_set.contains(&n) {
                if temp_g < grid.grid[n.x][n.y].g {
                    grid.grid[n.x][n.y].g = temp_g;
                    grid.grid[n.x][n.y].previous = Some(current);
                }
            } else {
                grid.grid[n.x][n.y].g = temp_g;
                grid.grid[n.x][n.y].previous = Some(current);
                grid.open_set.push(n);
                if n.x != grid.target.x || n.y != grid.target.y {
                    grid.grid[n.x][n.y].color = Color::get(OpenSet);
                }
            }
        }
    }
}
pub fn tick_path(grid: &mut Grid) {
    let item = grid.next_to_show;
    match item {
        Some(c) => {
            grid.grid[c.x][c.y].color = Color::get(PATH);
            grid.draw();
            grid.next_to_show = grid.grid[c.x][c.y].previous;
        }
        None => grid.stage = Stage::Done,
    }
}

pub fn solve(grid_ref: Rc<RefCell<Grid>>) {
    let mut grid = grid_ref.borrow_mut();
    let start_point = grid.start;
    grid.open_set.push(start_point);
    grid.stage = Stage::InProgress;
    set_all_h_scores(&mut grid);
    main_loop(grid_ref.clone());
}
pub fn main_loop(grid_ref: Rc<RefCell<Grid>>) {
    let cb = Closure::<dyn FnMut(_)>::new({
        let grid_ref = grid_ref.clone();
        move |_: web_sys::MouseEvent| {
            let mut grid = grid_ref.borrow_mut();
            match &grid.stage {
                Stage::InProgress => tick(&mut grid),
                Stage::DrawingPath => tick_path(&mut grid),
                _ => return,
            }
            main_loop(grid_ref.clone())
        }
    });
    web_sys::window()
        .unwrap()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
}

pub fn set_all_h_scores(grid: &mut Grid) {
    for row in &mut grid.grid {
        for cell in row {
            cell.set_h(&grid.target);
        }
    }
}

pub fn get_distance(cell1: &Position, cell2: &Position) -> i32 {
    let addent1 = (cell1.x as i32 - cell2.x as i32).abs();
    let addent2 = (cell1.y as i32 - cell2.y as i32).abs();
    return addent1 + addent2;
}
