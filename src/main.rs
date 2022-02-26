#[macro_use]
extern crate stdweb;
mod a_star;
mod canvas;
mod cell;
mod color;
mod event_listeners;
mod grid;
mod position;
use color::Color;
use grid::Grid;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    stdweb::initialize();
    let grid_ref = Rc::new(RefCell::new(Grid::new()));
    grid_ref.borrow_mut().draw();

    event_listeners::set_all_listeners(grid_ref.clone());
}
