pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub g: f32,
    pub is_wall: bool,
    pub color: String,
    pub previous: Option<Box<Cell>>,
}
impl Cell {
    pub fn new(x: i32, y: i32, color: String) -> Self {
        Self {
            x,
            y,
            g: 0.0,
            is_wall: false,
            color,
            previous: None,
        }
    }
}
