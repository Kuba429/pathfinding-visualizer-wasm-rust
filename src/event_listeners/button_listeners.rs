use crate::a_star;
use crate::grid::{stage, Grid};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

pub fn set_start_button_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let doc = web_sys::window().unwrap().document().unwrap();
    let start_button = doc.query_selector("#startButton").unwrap().unwrap();
    let cb = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        let stage = grid_ref.borrow_mut().stage;
        match stage {
            stage::idle => {
                disable_inputs();
                grid_ref.borrow_mut().set_diagonal();
                grid_ref.borrow_mut().set_comparing_h();
                a_star::solve(grid_ref.clone())
            }
            _ => return,
        }
    });
    start_button.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
    cb.forget();
}

pub fn set_reset_button_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let doc = web_sys::window().unwrap().document().unwrap();
    let reset_button = doc.query_selector("#resetButton").unwrap().unwrap();
    let cb = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        grid_ref.borrow_mut().reset();
    });
    reset_button.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

pub fn set_random_walls_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let doc = web_sys::window().unwrap().document().unwrap();
    let button = doc.query_selector("#randomWalls").unwrap().unwrap();
    let cb = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        let stage = grid_ref.borrow_mut().stage;
        match stage {
            stage::idle | stage::done => {
                let mut grid = grid_ref.borrow_mut();
                grid.fill_random_walls();
                grid.draw();
            }
            _ => return,
        }
    });
    button.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

pub fn disable_inputs() {
    let doc = web_sys::window().unwrap().document().unwrap();
    let input_elements: web_sys::NodeList = doc.query_selector_all("input").unwrap();
    for el in 0..input_elements.length() {
        let input_element: web_sys::Element = input_elements.item(el).unwrap().dyn_into().unwrap();
        input_element.set_attribute("disabled", "true");
    }
}
