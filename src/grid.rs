use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::document;

use crate::canvas::Canvas;
use crate::cell::Cell;
use crate::color::Color;
use crate::position::Position;
use crate::Color::{BLANK, GRID, START, TARGET};
pub struct Grid {
    pub canvas: Canvas,
    rows: i32,
    pub cell_size: f64,
    pub grid: Vec<Vec<Cell>>,
    pub start: Position,
    pub target: Position,
    pub open_set: Vec<Position>,
    pub closed_set: Vec<Position>,
    // allow_diagonals: bool,
    pub can_modify: bool,
    pub next_to_show: Option<Position>,
    pub stage: stage,
}

impl Grid {
    //clear the canvas
    pub fn draw(&mut self) {
        self.canvas.clear(&Color::get(GRID));

        //set start and targets colors
        self.grid[self.start.x][self.start.y].color = Color::get(START);
        self.grid[self.target.x][self.target.y].color = Color::get(TARGET);
        //draw cells
        for j in &self.grid {
            for i in j {
                self.canvas.draw(
                    (i.x as f64 * self.cell_size) + 0.5,
                    (i.y as f64 * self.cell_size) + 0.5,
                    self.cell_size - 1.0,
                    self.cell_size - 1.0,
                    &i.color,
                )
            }
        }
        // reset start and targets colors
        self.grid[self.start.x][self.start.y].color = Color::get(BLANK);
        self.grid[self.target.x][self.target.y].color = Color::get(BLANK);
    }
    pub fn make_new_start(&mut self, x: usize, y: usize) {
        self.start = Position::new(x, y);
        self.grid[x][y].make_not_wall();
    }
    pub fn make_new_target(&mut self, x: usize, y: usize) {
        self.target = Position::new(x, y);
        self.grid[x][y].make_not_wall();
    }
    pub fn fill_random_walls(&mut self) {
        self.grid = Self::setup_grid(&self.rows);
        for row in &mut self.grid {
            for cell in row {
                if cell.x as usize == self.target.x && cell.y as usize == self.target.y {
                    continue;
                };
                let random_number: u8 = js! {return Math.floor(Math.random()*100)}
                    .try_into()
                    .unwrap();
                if random_number < 20 {
                    cell.make_wall();
                }
            }
        }
    }
    pub fn reset(&mut self) {
        self.grid = Self::setup_grid(&self.rows);
        self.stage = stage::idle;
        self.draw();
    }
}
impl Grid {
    pub fn new() -> Self {
        let grid_size_range: stdweb::web::html_element::InputElement = document()
            .query_selector("#gridSizeRange")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let rows: i32 = grid_size_range.raw_value().parse().unwrap();
        let canvas = Canvas::new();
        let cell_size: f64 = canvas.element.width() as f64 / rows as f64;
        let grid = Self::setup_grid(&rows);
        let start = Position::new(0, 0);
        let target = Position::new(grid.len() - 1, grid.len() - 1);

        Self {
            canvas,
            rows,
            grid,
            cell_size,
            start,
            target,
            can_modify: true,
            open_set: Vec::new(),
            closed_set: Vec::new(),
            next_to_show: None,
            stage: stage::idle,
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

pub enum stage {
    in_progress,
    drawing_path,
    done,
    idle,
}
