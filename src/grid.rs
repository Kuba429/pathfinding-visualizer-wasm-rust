use js_sys::Math;
use wasm_bindgen::JsCast;

use crate::canvas::Canvas;
use crate::cell::Cell;
use crate::color::Color;
use crate::position::Position;
use crate::Color::{BLANK, GRID, START, TARGET};
pub struct Grid {
    pub canvas: Canvas,
    pub rows: i32,
    pub cell_size: f64,
    pub grid: Vec<Vec<Cell>>,
    pub start: Position,
    pub target: Position,
    pub open_set: Vec<Position>,
    pub closed_set: Vec<Position>,
    pub allow_diagonals: bool,
    pub compare_h: bool,
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
        self.reset();
        for row in &mut self.grid {
            for cell in row {
                if cell.x as usize == self.target.x && cell.y as usize == self.target.y {
                    continue;
                };
                let random_number: u8 = (Math::random() * 100.0).floor() as u8;
                if random_number < 20 {
                    cell.make_wall();
                }
            }
        }
    }
    pub fn set_diagonal(&mut self) {
        let doc = web_sys::window().unwrap().document().unwrap();
        self.allow_diagonals = doc
            .query_selector("#diagonalsCheckbox")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .checked();
    }
    pub fn set_comparing_h(&mut self) {
        let doc = web_sys::window().unwrap().document().unwrap();
        self.compare_h = doc
            .query_selector("#compare-h-checkbox")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .checked();
    }
    pub fn reset(&mut self) {
        self.grid = Self::setup_grid(&self.rows);
        self.stage = stage::idle;
        self.open_set = Vec::new();
        self.closed_set = Vec::new();

        self.cell_size = self.canvas.element.width() as f64 / self.rows as f64;

        self.start = Position::new(0, 0);
        self.target = Position::new(self.grid.len() - 1, self.grid.len() - 1);

        enable_inputs();

        self.draw();
    }
}
impl Grid {
    pub fn new() -> Self {
        let doc = web_sys::window().unwrap().document().unwrap();
        let grid_size_range: web_sys::HtmlInputElement = doc
            .query_selector("#gridSizeRange")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        let rows: i32 = grid_size_range.value().parse().unwrap();
        let canvas = Canvas::new();
        let cell_size: f64 = canvas.element.width() as f64 / rows as f64;
        let grid = Self::setup_grid(&rows);
        let start = Position::new(0, 0);
        let target = Position::new(grid.len() - 1, grid.len() - 1);
        let allow_diagonals = doc
            .query_selector("#diagonalsCheckbox")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .checked();
        Self {
            canvas,
            rows,
            grid,
            cell_size,
            start,
            target,
            allow_diagonals,
            compare_h: false,
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

#[derive(Clone, Copy)]
pub enum stage {
    in_progress,
    drawing_path,
    done,
    idle,
}

pub fn enable_inputs() {
    let doc = web_sys::window().unwrap().document().unwrap();
    let input_elements: web_sys::NodeList = doc.query_selector_all("input").unwrap();
    for el in 0..input_elements.length() {
        let input_element: web_sys::HtmlInputElement =
            input_elements.item(el).unwrap().dyn_into().unwrap();
        input_element.remove_attribute("disabled").unwrap();
    }
}
