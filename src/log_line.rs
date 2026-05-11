const DELIMITER: char = '|';

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    #[default]
    Debug,
    Trace,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LogLine {
    time_stamp: String,
    log_level: LogLevel,
    content: String,
}

impl LogLine {
    pub fn new(line: &str) -> Option<Self> {
        parse_log_line(line)
    }

    pub fn time_stamp(&self) -> &str {
        &self.time_stamp
    }
    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

fn parse_log_line(line: &str) -> Option<LogLine> {
    if line.trim().is_empty() {
        return None;
    }

    let mut splitted_line = line.split(&[DELIMITER]);

    let time_stamp = splitted_line.next()?.trim().to_string();
    let log_level = match splitted_line.next()?.trim() {
        "ERROR" => LogLevel::Error,
        "WARN" => LogLevel::Warn,
        "INFO" => LogLevel::Info,
        "DEBUG" => LogLevel::Debug,
        _ => LogLevel::Trace,
    };
    let content = splitted_line.next()?.trim().to_string();

    Some(LogLine {
        time_stamp,
        log_level,
        content,
    })
}
