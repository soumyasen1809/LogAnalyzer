#[derive(Debug, Default)]
pub struct FilterState {
    query: String,
    is_active: bool,
}

impl FilterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
    }

    pub fn set_active(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.is_active = false;
    }
}
