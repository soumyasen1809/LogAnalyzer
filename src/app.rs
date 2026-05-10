use crate::{log_store::LogStore, mode::Mode, search::SearchState};
use ratatui::widgets::ListState;
use rayon::prelude::*;
use std::sync::Arc;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct App {
    logs: Arc<LogStore>,
    mode: Mode,
    search: SearchState,
    list_state: ListState,
    rx: Option<oneshot::Receiver<Vec<usize>>>,
}

impl App {
    pub fn new(logs: Arc<LogStore>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            logs,
            mode: Mode::default(),
            search: SearchState::default(),
            list_state,
            rx: None,
        }
    }

    pub fn logs(&self) -> Arc<LogStore> {
        Arc::clone(&self.logs)
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn search(&self) -> &SearchState {
        &self.search
    }

    pub fn list_state(&self) -> ListState {
        self.list_state
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
    }

    pub fn exit_search_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn handle_search_char(&mut self, c: char) {
        self.search.push_char(c);
    }

    pub fn handle_search_backspace(&mut self) {
        self.search.pop_char();
    }

    pub fn clear_search(&mut self) {
        self.search.clear();
    }

    pub fn run_search(&mut self) {
        let query = self.search.query().to_lowercase();

        if query.is_empty() {
            return self.search.clear_matches();
        }

        let (tx, rx) = oneshot::channel();
        self.rx = Some(rx);

        let logs = Arc::clone(&self.logs);

        tokio::spawn(async move {
            let query_bytes = query.to_lowercase().into_bytes();
            let matches: Vec<usize> = logs
                .line_offsets
                .par_iter()
                .enumerate()
                .filter(|(_, (start, end))| {
                    let line = &logs.mmap[*start..*end];
                    let lower_line = line.to_ascii_lowercase();
                    if lower_line.len() < query_bytes.len() {
                        return false;
                    }
                    lower_line
                        .windows(query_bytes.len()) // Use windows for sub-slice search
                        .any(|window| window == query_bytes.as_slice())
                })
                .map(|(idx, _)| idx)
                .collect();
            let _ = tx.send(matches);
        });
    }

    pub fn receive_search_result(&mut self) {
        if let Some(matches) = self.rx.as_mut().and_then(|r| r.try_recv().ok()) {
            self.search.set_matches(matches);
            if let Some(first) = self.search.current_match_index() {
                self.list_state.select(Some(first));
            }
            self.rx = None;
        }
    }

    pub fn next_search_match(&mut self) {
        self.search.next_match();

        if let Some(idx) = self.search.current_match_index() {
            self.list_state.select(Some(idx));
        }
    }

    pub fn select_next(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        let next = (current + 1).min(self.logs().len().saturating_sub(1));
        self.list_state.select(Some(next));
    }

    pub fn select_previous(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(current.saturating_sub(1)));
    }
}
