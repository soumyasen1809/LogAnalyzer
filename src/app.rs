use crate::log_line::LogLine;
use std::collections::VecDeque;

const LOG_STORAGE_CAPACITY: usize = 10000;

#[derive(Debug)]
pub struct App {
    logs: VecDeque<LogLine>,
    scroll: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            logs: VecDeque::with_capacity(LOG_STORAGE_CAPACITY),
            scroll: 0,
        }
    }

    pub fn logs(&self) -> VecDeque<LogLine> {
        self.logs.clone()
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn push_logs(&mut self, new_line: LogLine) {
        if self.logs.len() >= LOG_STORAGE_CAPACITY {
            self.logs.pop_front();
        }
        self.logs.push_back(new_line);
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }
}
