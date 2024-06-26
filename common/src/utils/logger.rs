use std::time::Instant;
use colored::{Colorize, ColoredString};
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Logger {
    scope: String,
    level: i32
}

#[derive(Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub fn clear_screen(message: &str) {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\t{}\n", message.magenta().bold());
    
}

static START_TIME: OnceCell<Instant> = OnceCell::new();

impl Logger {

    pub fn new_leveled(scope: &str, level: i32) -> Logger {
        START_TIME.get_or_init(|| Instant::now());
        Logger {
            scope: scope.to_string(),
            level: level
        }
    }

    pub fn new (scope: &str) -> Logger {
        Logger::new_leveled(scope, 0)
    }

    fn elapsed_string(&self) -> String {
        let elapsed = START_TIME.get().unwrap().elapsed();
        let hours = elapsed.as_secs() / 3600;
        let minutes = (elapsed.as_secs() % 3600) / 60;
        let seconds = elapsed.as_secs() % 60;
        let milliseconds = elapsed.subsec_nanos() / 1_000_000;
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds)
    }

    fn log_line(&self, level_string: ColoredString, message: &str) {
        let elapsed = self.elapsed_string();
        let begin = format!("[{}] ", elapsed);
        let end = format!(" {: <10}: {}", self.scope, message);
        let line = format!("{}{}{}", begin.dimmed(), level_string, end);
        println!("{}", line);
    }

    fn log(&self, level: LogLevel, message: &str) {

        if self.level <= level as i32 { 
            match level {
                LogLevel::Debug => {
                    self.log_line("???".dimmed().bold(), message);
                },
                LogLevel::Info => {
                    self.log_line(" i ".cyan().bold(), message);
                },
                LogLevel::Warning => {
                    self.log_line(" # ".magenta().bold(), message);
                },
                LogLevel::Error => {
                    self.log_line("!!!".red().bold(), message);
                }
            }
        }
    }
    
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    
}