pub struct State {
    pub show_about: bool,
    pub show_log: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            show_about: false,
            show_log: false,
        }
    }
}