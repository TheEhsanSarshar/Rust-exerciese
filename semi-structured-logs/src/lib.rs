// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let mut level_string: &str;
    match level {
       LogLevel::Info => {
        level_string = "[INFO]: "
       },
       LogLevel::Error => {
        level_string = "[ERROR]: "
       },
       LogLevel::Warning => {
        level_string = "[WARNING]: "
       },
       LogLevel::Debug => {
        level_string = "[DEBUG]: "
       }
    }
    [level_string, message].concat()
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
