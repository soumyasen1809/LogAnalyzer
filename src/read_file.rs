use crate::{errors::Errors, log_line::LogLine};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc,
};

pub async fn read_log_file(path: &str, tx: mpsc::Sender<LogLine>) -> Result<(), Errors> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if let Some(log_line) = LogLine::new(&line) {
            tx.send(log_line).await?;
        }
    }

    Ok(())
}
