use crate::a_star;
use crate::grid::{stage, Grid};
use crate::stdweb::web::IEventTarget;
use crate::stdweb::web::IParentNode;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::unstable::TryInto;
use stdweb::web::event::ClickEvent;
use stdweb::web::{document, html_element, IElement};

pub fn set_start_button_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let start_button = document().query_selector("#startButton").unwrap().unwrap();
    start_button.add_event_listener({
        move |_e: ClickEvent| {
            let stage = grid_ref.borrow_mut().stage;
            match stage {
                stage::idle => {
                    disable_inputs();
                    grid_ref.borrow_mut().set_diagonal();
                    a_star::solve(grid_ref.clone())
                }
                _ => return,
            }
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
            let stage = grid_ref.borrow_mut().stage;
            match stage {
                stage::idle | stage::done => {
                    let mut grid = grid_ref.borrow_mut();
                    grid.fill_random_walls();
                    grid.draw();
                }
                _ => return,
            }
        }
    });
}

pub fn disable_inputs() {
    let input_elements: stdweb::web::NodeList = document().query_selector_all("input").unwrap();
    for el in input_elements {
        let input_element: html_element::InputElement = el.try_into().unwrap();
        input_element.set_attribute("disabled", "true").unwrap();
    }
}
