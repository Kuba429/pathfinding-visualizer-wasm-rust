use crate::a_star::get_distance;
use crate::color::Color;
use crate::color::Color::{BLANK, GRID};
use crate::{grid::Grid, position::Position};
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub g: i32,
    pub h: i32,
    pub is_wall: bool,
    pub color: String,
    pub previous: Option<Position>,
}

impl Cell {
    pub fn make_wall(&mut self) {
        self.is_wall = true;
        self.color = Color::get(GRID);
    }
    pub fn make_not_wall(&mut self) {
        self.is_wall = false;
        self.color = Color::get(BLANK);
    }
    pub fn get_neighbors(&self, grid: &Grid) -> Vec<Position> {
        let x = self.x as usize;
        let y = self.y as usize;
        let mut neighbors: Vec<Position> = Vec::new();
        let mut potential_neighbors = Vec::new();
        let last_i = grid.grid.len() - 1;
        if x > 0 {
            potential_neighbors.push(Position::new(x - 1, y));
        }
        if x < last_i {
            potential_neighbors.push(Position::new(x + 1, y))
        }
        if y > 0 {
            potential_neighbors.push(Position::new(x, y - 1))
        }
        if y < last_i {
            potential_neighbors.push(Position::new(x, y + 1))
        };
        if grid.allow_diagonals {
            if x > 0 && y > 0 {
                potential_neighbors.push(Position::new(x - 1, y - 1));
            }
            if x > 0 && y < last_i {
                potential_neighbors.push(Position::new(x - 1, y + 1));
            }
            if x < last_i && y > 0 {
                potential_neighbors.push(Position::new(x + 1, y - 1));
            }
            if x < last_i && y < last_i {
                potential_neighbors.push(Position::new(x + 1, y + 1));
            }
        }

        for i in potential_neighbors {
            if !grid.grid[i.x][i.y].is_wall {
                neighbors.push(i);
            }
        }
        return neighbors;
    }
}
impl Cell {
    pub fn set_h(&mut self, target: &Position) {
        self.h = get_distance(&Position::new(self.x as usize, self.y as usize), target);
    }
    pub fn f(&self) -> i32 {
        return self.g + self.h;
    }
}
impl Cell {
    pub fn new(x: i32, y: i32, color: String) -> Self {
        Self {
            x,
            y,
            g: 0,
            h: 0,
            is_wall: false,
            color,
            previous: None,
        }
    }
}
