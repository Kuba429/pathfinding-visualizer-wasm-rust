use crate::grid::{Grid, Stage};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

pub fn set_canvas_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let doc = web_sys::window().unwrap().document().unwrap();
    let canvas: web_sys::HtmlCanvasElement = doc
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    let main_form: web_sys::HtmlFormElement = doc
        .query_selector("#mainForm")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let cb = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
        let form_data = web_sys::FormData::new_with_form(&main_form).unwrap();
        let mut grid = grid_ref.borrow_mut();
        match grid.stage {
            Stage::Idle => (),
            _ => return,
        }
        let cell_size = grid.cell_size;
        let mut x = (e.offset_x() / cell_size as i32) as usize;
        let mut y = (e.offset_y() / cell_size as i32) as usize;
        // cap the x and y values to the grid; caused some bugs
        if x >= grid.grid.len() {
            x = grid.grid.len() - 1;
        }
        if y >= grid.grid.len() {
            y = grid.grid.len() - 1;
        }
        let object = form_data.get("object").as_string().unwrap();

        if object == "wall" {
            grid.grid[x][y].make_wall();
        } else if object == "eraseWall" {
            grid.grid[x][y].make_not_wall();
        } else if object == "startPoint" {
            grid.make_new_start(x, y);
        } else if object == "destination" {
            grid.make_new_target(x, y);
        }
        grid.draw();
    });
    canvas
        .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
}

pub fn set_canvas_wall_drawing_listener(grid_ref: Rc<RefCell<Grid>>) {
    let doc = web_sys::window().unwrap().document().unwrap();
    let canvas = doc.query_selector("#canvas").unwrap().unwrap();
    let main_form: web_sys::HtmlFormElement = doc
        .query_selector("#mainForm")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    let mouse = Rc::new(RefCell::new(MouseState { is_down: false }));
    {
        let mouse = mouse.clone();
        let cb = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            mouse.borrow_mut().set_state(true)
        });
        doc.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }
    {
        let mouse = mouse.clone();
        let cb = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            mouse.borrow_mut().set_state(false)
        });
        doc.add_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }
    {
        let mouse = mouse.clone();
        let cb = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
            if !mouse.borrow_mut().is_down {
                return;
            }
            let mut grid = grid_ref.borrow_mut();
            match grid.stage {
                Stage::Idle => (),
                _ => return,
            }
            let form_data = web_sys::FormData::new_with_form(&main_form).unwrap();
            let cell_size = grid.cell_size;
            let mut x = (e.offset_x() / cell_size as i32) as usize;
            let mut y = (e.offset_y() / cell_size as i32) as usize;
            // cap the x and y values to the grid; caused some bugs
            if x >= grid.grid.len() {
                x = grid.grid.len() - 1;
            }
            if y >= grid.grid.len() {
                y = grid.grid.len() - 1;
            }
            let object = form_data.get("object").as_string().unwrap();

            if object == "wall" {
                grid.grid[x][y].make_wall();
            } else if object == "eraseWall" {
                grid.grid[x][y].make_not_wall();
            } else if object == "startPoint" {
                grid.make_new_start(x, y);
            } else if object == "destination" {
                grid.make_new_target(x, y);
            }
            grid.draw();
        });
        canvas
            .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }
}
struct MouseState {
    pub is_down: bool,
}
impl MouseState {
    pub fn set_state(&mut self, new: bool) {
        self.is_down = new;
    }
}
