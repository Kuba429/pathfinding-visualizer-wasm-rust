#[macro_use]
extern crate stdweb;
mod canvas;
mod cell;
mod color;
mod grid;
mod position;
use color::Color;
use grid::Grid;
fn main() {
    stdweb::initialize();
    let canvas = canvas::Canvas::new();
    let mut grid = Grid::new(&canvas);
    grid.draw();
}
