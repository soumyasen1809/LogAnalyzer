const DELIMITER: char = '|';

#[derive(Debug, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone)]
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
