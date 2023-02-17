use crate::grid::{stage, Grid};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn set_size_range_oninput(grid_ref: Rc<RefCell<Grid>>) {
    let doc = web_sys::window().unwrap().document().unwrap();
    let input_element: web_sys::HtmlInputElement = doc
        .query_selector("#gridSizeRange")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    let cb = Closure::<dyn FnMut(_)>::new(move |e: web_sys::InputEvent| {
        let mut grid = grid_ref.borrow_mut();
        match grid.stage {
            stage::idle => {
                let target: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
                let value: i32 = target.value().parse().unwrap();
                grid.rows = value;
                grid.reset();
            }
            _ => return,
        }
    });
    input_element
        .add_event_listener_with_callback("input", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
}
