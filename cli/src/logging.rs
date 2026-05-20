use chrono::Local;
use std::fs::{File, create_dir_all};
use std::io::Write;

pub struct Logger {
    file: File,
    verbose: u8,
}

impl Logger {
    pub fn new(log_dir: &str, prefix: &str, verbose: u8) -> std::io::Result<Self> {
        create_dir_all(log_dir)?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}/{}_{}.log", log_dir, prefix, timestamp);
        let file = File::create(filename)?;
        Ok(Logger { file, verbose })
    }
    pub fn log(&mut self, msg: &str, level: u8) {
        if level <= self.verbose {
            let _ = writeln!(self.file, "{}", msg);
        }
    }
}
