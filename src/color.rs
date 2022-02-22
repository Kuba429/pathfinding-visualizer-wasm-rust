pub enum Color {
    GRID,
    BLANK,
}
impl Color {
    pub fn get(value: Color) -> String {
        match value {
            Color::GRID => String::from("#000000"),
            Color::BLANK => String::from("#ffffff"),
        }
    }
}
