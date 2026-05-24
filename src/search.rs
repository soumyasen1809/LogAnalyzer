use crate::log_store::LogStore;
use rayon::prelude::*;
use regex::bytes::RegexBuilder;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct SearchState {
    query: String,
    matches: Vec<usize>,
    current_match: usize,
    is_regex_search: bool,
}

impl SearchState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn is_regex_search(&self) -> bool {
        self.is_regex_search
    }

    pub fn set_is_regex_search(&mut self, is_regex_on: bool) {
        self.is_regex_search = is_regex_on;
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

    pub fn run_search(
        &mut self,
        logs: Arc<LogStore>,
        tx: tokio::sync::oneshot::Sender<Vec<usize>>,
    ) {
        if self.query.is_empty() {
            return self.clear_matches();
        }

        if self.is_regex_search {
            self.run_regex_search(&self.query, logs, tx)
        } else {
            self.run_non_regex_search(&self.query, logs, tx)
        }
    }

    fn run_regex_search(
        &self,
        query: &str,
        logs: Arc<LogStore>,
        tx: tokio::sync::oneshot::Sender<Vec<usize>>,
    ) {
        let regex = match RegexBuilder::new(query).case_insensitive(true).build() {
            Ok(r) => r,
            Err(_) => {
                let _ = tx.send(Vec::new());
                return;
            }
        };

        tokio::spawn(async move {
            let matches: Vec<usize> = logs
                .line_offsets
                .par_iter()
                .enumerate()
                .filter(|(_, (start, end))| {
                    let line = &logs.mmap[*start..*end];
                    regex.is_match(line)
                })
                .map(|(idx, _)| idx)
                .collect();
            let _ = tx.send(matches);
        });
    }

    fn run_non_regex_search(
        &self,
        query: &str,
        logs: Arc<LogStore>,
        tx: tokio::sync::oneshot::Sender<Vec<usize>>,
    ) {
        let query_bytes = query.to_lowercase().into_bytes();
        tokio::spawn(async move {
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
}
