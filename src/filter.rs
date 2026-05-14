#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FilterOp {
    #[default]
    And,
    Or,
}

#[derive(Debug, Default)]
pub struct FilterState {
    query: String,
    is_active: bool,
    op: FilterOp,
}

impl FilterState {
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

    pub fn op(&self) -> FilterOp {
        self.op
    }

    pub fn set_op(&mut self, op: FilterOp) {
        self.op = op
    }

    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.is_active = false;
    }
}
