mod button_onclick;
mod canvas;
use crate::grid::Grid;
use std::cell::RefCell;
use std::rc::Rc;

pub fn set_all_listeners(grid_ref: Rc<RefCell<Grid>>) {
    button_onclick::set_start_button_onclick(grid_ref.clone());
    button_onclick::set_random_walls_onclick(grid_ref.clone());
    canvas::set_canvas_onclick(grid_ref.clone());
}
