#[macro_use]
extern crate stdweb;
mod canvas;
mod cell;
mod color;
mod grid;
use color::Color;
use grid::Grid;
fn main() {
    stdweb::initialize();
    let canvas = canvas::Canvas::new();
    let grid = Grid::new(&canvas);
    grid.draw();
}
