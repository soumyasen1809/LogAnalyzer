use crate::{log_line::LogLine, mode::Mode, search::SearchState};
use ratatui::widgets::ListState;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};
use tokio::sync::oneshot;

const LOG_STORAGE_CAPACITY: usize = 10_000;

#[derive(Debug, Default)]
pub struct App {
    logs: Arc<RwLock<VecDeque<LogLine>>>,
    mode: Mode,
    search: SearchState,
    list_state: ListState,
    rx: Option<oneshot::Receiver<Vec<usize>>>,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            logs: Arc::new(RwLock::new(VecDeque::with_capacity(LOG_STORAGE_CAPACITY))),
            list_state,
            ..Default::default()
        }
    }

    pub fn logs(&self) -> Arc<RwLock<VecDeque<LogLine>>> {
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

    pub fn push_logs(&mut self, log: LogLine) {
        if let Ok(mut logs) = self.logs.write() {
            if logs.len() >= LOG_STORAGE_CAPACITY {
                logs.pop_front();
            }

            logs.push_back(log);

            if self.list_state.selected().is_none() {
                self.list_state.select(Some(0));
            }
        }
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
            let matches = logs.read().map_or_else(
                |_| Vec::new(),
                |log| {
                    log.iter()
                        .enumerate()
                        .filter(|(_, log)| log.content().to_lowercase().contains(&query))
                        .map(|(idx, _)| idx)
                        .collect()
                },
            );
            let _ = tx.send(matches);
        });
    }

    pub fn receive_search_result(&mut self) {
        if let Some(ref mut receiver) = self.rx {
            match receiver.try_recv() {
                Ok(matches) => {
                    self.search.set_matches(matches);
                    if let Some(first) = self.search.current_match_index() {
                        self.list_state.select(Some(first));
                    }

                    self.rx = None;
                }
                Err(oneshot::error::TryRecvError::Empty) => {}
                Err(oneshot::error::TryRecvError::Closed) => {
                    self.rx = None;
                }
            }
        }
    }

    pub fn next_match(&mut self) {
        self.search.next_match();

        if let Some(idx) = self.search.current_match_index() {
            self.list_state.select(Some(idx));
        }
    }

    pub fn select_next(&mut self) {
        if let Ok(logs) = self.logs.read() {
            let current = self.list_state.selected().unwrap_or(0);
            let next = (current + 1).min(logs.len().saturating_sub(1));
            self.list_state.select(Some(next));
        }
    }

    pub fn select_previous(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(current.saturating_sub(1)));
    }
}
