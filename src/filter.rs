#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FilterOp {
    #[default]
    And,
    Or,
}

#[derive(Debug, Default)]
pub struct FilterState {
    query: String,
    is_editing: bool,
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

    pub fn is_editing(&self) -> bool {
        self.is_editing
    }

    pub fn set_is_editing(&mut self, is_editing: bool) {
        self.is_editing = is_editing;
    }

    pub fn op(&self) -> FilterOp {
        self.op
    }

    pub fn set_op(&mut self, op: FilterOp) {
        self.op = op
    }

    pub fn change_op(&mut self) {
        let new_op = match self.op {
            FilterOp::And => FilterOp::Or,
            FilterOp::Or => FilterOp::And,
        };
        self.set_op(new_op);
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
