use wasm_bindgen::{JsCast, JsValue};
pub struct Canvas {
    pub element: web_sys::HtmlCanvasElement,
    pub ctx: web_sys::CanvasRenderingContext2d,
}
impl Canvas {
    pub fn clear(&self, color: &str) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.element.width() as f64,
            self.element.height() as f64,
        );
        self.ctx.set_fill_style(&JsValue::from_str(color));
        self.ctx.fill_rect(
            0.0,
            0.0,
            self.element.width() as f64,
            self.element.height() as f64,
        )
    }
    pub fn draw(&self, x: f64, y: f64, width: f64, height: f64, color: &String) {
        self.ctx.set_fill_style(&JsValue::from_str(color));
        self.ctx.fill_rect(x, y, width, height);
    }
}
// methods that don't take "self"
impl Canvas {
    pub fn new() -> Canvas {
        let doc = web_sys::window().unwrap().document().unwrap();
        let element: web_sys::HtmlCanvasElement = doc
            .query_selector("#canvas")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        let ctx: web_sys::CanvasRenderingContext2d = element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        Canvas { element, ctx }
    }
}
