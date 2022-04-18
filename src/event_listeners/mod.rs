mod button_listeners;
mod canvas_listeners;
mod input_listeners;
use crate::grid::Grid;
use std::cell::RefCell;
use std::rc::Rc;

pub fn set_all_listeners(grid_ref: Rc<RefCell<Grid>>) {
    button_listeners::set_start_button_onclick(grid_ref.clone());
    button_listeners::set_random_walls_onclick(grid_ref.clone());
    button_listeners::set_reset_button_onclick(grid_ref.clone());
    canvas_listeners::set_canvas_onclick(grid_ref.clone());
    canvas_listeners::set_canvas_wall_drawing_listener(grid_ref.clone());
    input_listeners::set_size_range_oninput(grid_ref.clone());
    input_listeners::toggle_compare_h(grid_ref.clone());
}
