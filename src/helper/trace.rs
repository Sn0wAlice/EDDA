use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::Write;

use colored::Colorize;

use serde_json::{json, Value};

// create a global vector to store the logs
pub static mut LOGS: Vec<Value> = Vec::new();


pub fn is_debug_mode() -> bool {
  if std::env::args().any(|arg| arg == "--debug") {
    return true;
  }
  return false;
}

pub fn is_debug_mode_enable() -> String {
  if std::env::args().any(|arg| arg == "--debug") {
    return "enable".to_string();
  }
  return "disable".to_string();
}


pub fn logs(mut technical_message:Value, level:i8) {

  // check if technical_message contains a message
  if technical_message["message"].is_string() {
    let message = technical_message["message"].as_str().unwrap();
    let mut level_value = "DEBUG".to_string().blue();
    if level == 1 {
      level_value = "INFO".to_string().green();
    } else if level == 2 {
      level_value = "WARN".to_string().yellow();
    } else if level == 3 {
      level_value = "ERROR".to_string().red();
    }  else if level == 4 {
      level_value = "EVENT".to_string().truecolor(255, 204, 255)
    } else  if level == 27 {
      level_value = "SECURITY 🔪".to_string().truecolor(255, 0, 0)
    } else {
      // default to debug, but is only allowed in development, check if arfs --debug is set in command line
      if std::env::args().any(|arg| arg == "--debug") {
        // no problem
      } else {
        return;
      }
    }

    // [2021-08-10 12:00:00:000 0000] INFO - message
    let log_message = format!("[{}] {} - {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f %z").to_string().truecolor(169, 169, 169), level_value, message);
    println!("{}", log_message);
  }

}
