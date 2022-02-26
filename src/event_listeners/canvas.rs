use crate::grid::Grid;
use crate::stdweb::traits::IMouseEvent;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::web::event::ClickEvent;
use stdweb::web::{document, FormData, FormDataEntry, IEventTarget, IParentNode};

pub fn set_canvas_onclick(grid_ref: Rc<RefCell<Grid>>) {
    let canvas = document().query_selector("#canvas").unwrap().unwrap();
    let main_form = document().query_selector("#mainForm").unwrap().unwrap();
    canvas.add_event_listener({
        move |e: ClickEvent| {
            let form_data = FormData::from_element(&main_form).unwrap();
            let mut grid = grid_ref.borrow_mut();
            if !grid.can_modify {
                return;
            };
            let cell_size = grid.cell_size;
            let x = (e.offset_x() / cell_size) as usize;
            let y = (e.offset_y() / cell_size) as usize;
            let object = form_data.get("object").unwrap();
            // all FormDataEntries
            let start_point = FormDataEntry::String("startPoint".to_string());
            let destination = FormDataEntry::String("destination".to_string());
            let wall = FormDataEntry::String("wall".to_string());
            let erase_wall = FormDataEntry::String("eraseWall".to_string());

            if object == wall {
                grid.grid[x][y].make_wall();
            } else if object == erase_wall {
                grid.grid[x][y].make_not_wall();
            } else if object == start_point {
                grid.make_new_start(x, y);
            } else if object == destination {
                grid.make_new_target(x, y);
            }
            grid.draw();
        }
    });
}
