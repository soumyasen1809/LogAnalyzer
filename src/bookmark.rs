use ratatui::widgets::ListState;

#[derive(Debug, Default)]
pub struct BookMark {
    indices: Vec<usize>,
    state: ListState,
}

impl BookMark {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            indices: Vec::new(),
            state,
        }
    }

    pub fn indices(&self) -> &[usize] {
        &self.indices
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn add_bookmark(&mut self, index: usize) {
        if !self.indices.contains(&index) {
            self.indices.push(index);
            self.indices.sort_unstable();
        }
    }

    pub fn remove_bookmark(&mut self, index: usize) {
        if let Some(pos) = self.indices.iter().position(|&i| i == index) {
            self.indices.remove(pos);
        }
    }

    pub fn select_next_bookmark(&mut self) {
        if self.indices.is_empty() {
            return;
        }

        let index = self
            .state
            .selected()
            .map(|i| (i + self.indices.len() - 1) % self.indices.len())
            .unwrap_or(0);
        self.state.select(Some(index));
    }

    pub fn select_previous_bookmark(&mut self) {
        if self.indices.is_empty() {
            return;
        }

        let index = self
            .state
            .selected()
            .map(|i| (i + 1) % self.indices.len())
            .unwrap_or(0);
        self.state.select(Some(index));
    }
}
