#[derive(Debug, Default, Clone)]
pub struct HighlightState {
    query: String,
    is_editing: bool,
    is_active: bool,
}

impl HighlightState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn is_editing(&self) -> bool {
        self.is_editing
    }

    pub fn set_is_editing(&mut self, is_editing: bool) {
        self.is_editing = is_editing;
    }

    pub fn push_char(&mut self, ch: char) {
        self.query.push(ch);
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.is_active = false;
    }
}
