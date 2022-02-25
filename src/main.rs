#[macro_use]
extern crate stdweb;
mod a_star;
mod canvas;
mod cell;
mod color;
mod grid;
mod position;
use color::Color;
use grid::Grid;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::IMouseEvent;
use stdweb::web::event::ClickEvent;
use stdweb::web::{document, FormData, FormDataEntry, IEventTarget, IParentNode};
fn main() {
    stdweb::initialize();
    let canvas = canvas::Canvas::new();
    let grid = Rc::new(RefCell::new(Grid::new()));
    grid.borrow_mut().draw();

    set_onclick(grid.clone());
    start_listener(grid.clone());
}
pub fn start_listener(grid: Rc<RefCell<grid::Grid>>) {
    let start_button = document().query_selector("#startButton").unwrap().unwrap();
    start_button.add_event_listener({
        move |e: ClickEvent| {
            let mut grid = grid.borrow_mut();
            if !grid.can_modify {
                return;
            }
            grid.can_modify = false;
            a_star::solve(&mut grid);
        }
    });
}

pub fn set_onclick(grid: Rc<RefCell<grid::Grid>>) {
    let canvas = document().query_selector("#canvas").unwrap().unwrap();
    canvas.add_event_listener({
        move |e: ClickEvent| {
            let main_form = document().query_selector("#mainForm").unwrap().unwrap();
            let form_data = FormData::from_element(&main_form).unwrap();
            let mut grid = grid.borrow_mut();
            let cell_size = grid.cell_size;
            let x = (e.offset_x() / cell_size) as usize;
            let y = (e.offset_y() / cell_size) as usize;
            let object = form_data.get("object").unwrap();
            // all FormDataEntries
            let startPoint = FormDataEntry::String("startPoint".to_string());
            let destination = FormDataEntry::String("destination".to_string());
            let wall = FormDataEntry::String("wall".to_string());
            let eraseWall = FormDataEntry::String("eraseWall".to_string());

            if object == wall {
                grid.grid[x][y].make_wall();
            } else if object == eraseWall {
                grid.grid[x][y].make_not_wall();
            } else if object == startPoint {
                grid.make_new_start(x, y);
            } else if object == destination {
                grid.make_new_target(x, y);
            }
            grid.draw();
        }
    });
}
