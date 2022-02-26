use crate::a_star;
use crate::grid::Grid;
use crate::stdweb::web::IEventTarget;
use crate::stdweb::web::IParentNode;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::web::document;
use stdweb::web::event::ClickEvent;

pub fn set_start_button_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let start_button = document().query_selector("#startButton").unwrap().unwrap();
    start_button.add_event_listener({
        move |_e: ClickEvent| {
            let grid = grid_ref.clone();
            if !grid.borrow_mut().can_modify {
                return;
            }
            a_star::solve(grid);
        }
    });
}

pub fn set_reset_button_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let reset_button = document().query_selector("#resetButton").unwrap().unwrap();
    reset_button.add_event_listener({
        move |_e: ClickEvent| {
            grid_ref.borrow_mut().reset();
        }
    });
}

pub fn set_random_walls_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let button = document().query_selector("#randomWalls").unwrap().unwrap();
    button.add_event_listener({
        move |_e: ClickEvent| {
            let mut grid = grid_ref.borrow_mut();
            grid.fill_random_walls();
            grid.draw();
        }
    });
}
