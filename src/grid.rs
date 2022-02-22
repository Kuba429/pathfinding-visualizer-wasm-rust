use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;

use crate::canvas::Canvas;
use crate::cell::Cell;
use crate::color::Color;
use crate::position::Position;
use crate::Color::{BLANK, GRID, START, TARGET};
pub struct Grid<'a> {
    canvas: &'a Canvas,
    rows: i32,
    cell_size: f64,
    grid: Vec<Vec<Cell>>,
    start: Position,
    target: Position,
    // open_set: Vec<Cell>,
    // closed_set: Vec<Cell>,
    // allow_diagonals: bool,
    // can_modify: bool,
}

impl Grid<'_> {
    //clear the canvas
    pub fn draw(&self) {
        self.canvas.clear(&Color::get(GRID));
        //draw cells
        for j in &self.grid {
            for i in j {
                self.canvas.draw(
                    (i.x as f64 * self.cell_size) + 1.0,
                    (i.y as f64 * self.cell_size) + 1.0,
                    self.cell_size - 2.0,
                    self.cell_size - 2.0,
                    &i.color,
                )
            }
        }
    }
}
impl<'a> Grid<'a> {
    pub fn new(canvas: &'a Canvas) -> Self {
        let grid_size_range: stdweb::web::html_element::InputElement = document()
            .query_selector("#gridSizeRange")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let rows: i32 = grid_size_range.raw_value().parse().unwrap();
        let cell_size: f64 = canvas._element.width() as f64 / rows as f64;
        let mut grid = Self::setup_grid(&rows);
        let start = Position::new(0, 0);
        let target = Position::new(grid.len() - 1, grid.len() - 1);
        grid[start.x][start.y].color = Color::get(START);
        grid[target.x][target.y].color = Color::get(TARGET);

        Self {
            canvas,
            rows,
            grid,
            cell_size,
            start,
            target,
        }
    }
    pub fn setup_grid(rows: &i32) -> Vec<Vec<Cell>> {
        let mut grid = Vec::new();
        for i in 0..*rows {
            grid.push(Vec::new());
            for j in 0..=*rows {
                grid[(i) as usize].push(Cell::new(i, j, Color::get(BLANK)));
            }
        }

        return grid;
    }
}
