#[derive(Debug, Default)]
pub struct SearchState {
    query: String,
    matches: Vec<usize>,
    current_match: usize,
}

impl SearchState {
    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.current_match = 0;
    }

    pub fn matches(&self) -> &[usize] {
        &self.matches
    }

    pub fn set_matches(&mut self, matches: Vec<usize>) {
        self.matches = matches;
        self.current_match = 0;
    }

    pub fn clear_matches(&mut self) {
        self.matches.clear();
        self.current_match = 0;
    }

    pub fn has_matches(&self) -> bool {
        !self.matches.is_empty()
    }

    pub fn current_match_index(&self) -> Option<usize> {
        self.matches.get(self.current_match).copied()
    }

    pub fn next_match(&mut self) {
        if self.matches.is_empty() {
            return;
        }

        self.current_match = (self.current_match + 1) % self.matches.len();
    }
}
