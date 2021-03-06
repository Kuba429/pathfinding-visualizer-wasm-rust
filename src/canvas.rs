use crate::stdweb::unstable::TryInto;
use stdweb::traits::*;
use stdweb::web::{document, html_element};
pub struct Canvas {
    pub element: html_element::CanvasElement,
    pub ctx: stdweb::web::CanvasRenderingContext2d,
    // pub width: i32,
    // pub height: u32,
}
impl Canvas {
    pub fn clear(&self, color: &str) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.element.width() as f64,
            self.element.height() as f64,
        );
        self.ctx.set_fill_style_color(color);
        self.ctx.fill_rect(
            0.0,
            0.0,
            self.element.width() as f64,
            self.element.height() as f64,
        )
    }
    pub fn draw(&self, x: f64, y: f64, width: f64, height: f64, color: &String) {
        self.ctx.set_fill_style_color(&color);
        self.ctx.fill_rect(x, y, width, height);
    }
}
// methods that don't take "self"
impl Canvas {
    pub fn new() -> Canvas {
        let element: html_element::CanvasElement = document()
            .query_selector("#canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let ctx = element.get_context().unwrap();
        Canvas {
            element,
            ctx,
            // width: i32::from(&element.width()),
            // height: &element.height(),
        }
    }
}
