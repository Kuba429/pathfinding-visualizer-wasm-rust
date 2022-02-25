pub enum Color {
    GRID,
    BLANK,
    START,
    TARGET,
    PATH,
    OPEN_SET,
    CLOSED_SET,
}
impl Color {
    pub fn get(value: Color) -> String {
        match value {
            Color::GRID => String::from("#000000"),
            Color::BLANK => String::from("#ffffff"),
            Color::START => String::from("#0000ff"),
            Color::TARGET => String::from("#0000ff"),
            Color::PATH => String::from("#0000ff"),
            Color::OPEN_SET => String::from("#00ff00"),
            Color::CLOSED_SET => String::from("#ff0000"),
        }
    }
}
