pub enum Color {
    GRID,
    BLANK,
    START,
    TARGET,
    PATH,
    OpenSet,
    ClosedSet,
}
impl Color {
    pub fn get(value: Color) -> String {
        match value {
            Color::GRID => String::from("#000000"),
            Color::BLANK => String::from("#f7f5f2"),
            Color::START => String::from("#077dff"),
            Color::TARGET => String::from("#077dff"),
            Color::PATH => String::from("#077dff"),
            Color::OpenSet => String::from("#77D970"),
            Color::ClosedSet => String::from("#E02401"),
        }
    }
}
