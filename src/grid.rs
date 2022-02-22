use stdweb::traits::*;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::Color::GRID;
pub struct Grid<'a> {
    canvas: &'a Canvas,
}

impl Grid<'_> {
    pub fn draw(&self) {
        self.canvas.draw(5.0, 9.0, 30.0, 30.0, Color::get(GRID));
        self.canvas.clear(&Color::get(GRID));
    }
}
impl<'a> Grid<'a> {
    pub fn new(canvas: &'a Canvas) -> Self {
        Self { canvas }
    }
}
