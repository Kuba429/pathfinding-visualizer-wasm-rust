use crate::grid::{stage, Grid};
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::{
    traits::IEvent,
    unstable::TryInto,
    web::{document, html_element::InputElement, IEventTarget, IParentNode},
};

pub fn set_size_range_oninput(grid_ref: Rc<RefCell<Grid>>) {
    let input_element: InputElement = document()
        .query_selector("#gridSizeRange")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    input_element.add_event_listener({
        move |e: stdweb::web::event::InputEvent| {
            let mut grid = grid_ref.borrow_mut();
            match grid.stage {
                stage::idle => {
                    let target: InputElement = e.target().unwrap().try_into().unwrap();
                    let value = target.raw_value().parse().unwrap();
                    grid.rows = value;
                    grid.reset();
                }
                _ => return,
            }
        }
    });
}

pub fn toggle_compare_h(grid_ref: Rc<RefCell<Grid>>) {
    let input_element: InputElement = document()
        .query_selector("#compare-h-checkbox")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    input_element.add_event_listener({
        move |e: stdweb::web::event::InputEvent| {
            let mut grid = grid_ref.borrow_mut();
            match grid.stage {
                stage::idle => {
                    let target: InputElement = e.target().unwrap().try_into().unwrap();
                    grid.compare_h = true;
                }
                _ => return,
            }
        }
    });
}
