use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, ThreadId};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum LogLevel {
  Debug,
  #[default]
  Info,
  Warning,
  Error,
  Critical,
}

impl std::fmt::Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LogLevel::Debug => write!(f, "DEBUG"),
      LogLevel::Info => write!(f, "INFO"),
      LogLevel::Warning => write!(f, "WARN"),
      LogLevel::Error => write!(f, "ERROR"),
      LogLevel::Critical => write!(f, "CRIT"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct LogMessage {
  pub thread_id: ThreadId,
  pub thread_name: String,
  pub timestamp: u64,
  pub level: LogLevel,
  pub source: String,
  pub message: String,
  pub context: Option<HashMap<String, String>>,
  pub file: Option<&'static str>,
  pub line: Option<u32>,
}

impl LogMessage {
  pub fn new(
    thread_name: String,
    level: LogLevel,
    source: String,
    message: String,
  ) -> Self {
    Self {
      thread_id: thread::current().id(),
      thread_name,
      timestamp: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64,
      level,
      source,
      message,
      context: None,
      file: None,
      line: None,
    }
  }

  pub fn with_context(mut self, context: HashMap<String, String>) -> Self {
    self.context = Some(context);
    self
  }

  pub fn with_location(mut self, file: &'static str, line: u32) -> Self {
    self.file = Some(file);
    self.line = Some(line);
    self
  }
}

pub trait LogOutput: Send + Sync {
  fn write(&self, message: &LogMessage);
}

pub struct ConsoleOutput {
  use_colors: bool,
}

impl ConsoleOutput {
  pub fn new(use_colors: bool) -> Self {
    Self { use_colors }
  }

  fn format_message(&self, message: &LogMessage) -> String {
    let timestamp = message.timestamp % 86400000; // Get time within day in ms
    let hours = timestamp / 3600000;
    let minutes = (timestamp % 3600000) / 60000;
    let seconds = (timestamp % 60000) / 1000;
    let millis = timestamp % 1000;

    let time_str = format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis);

    let level_str = if self.use_colors {
      match message.level {
        LogLevel::Debug => "\x1b[36mDEBUG\x1b[0m",
        LogLevel::Info => "\x1b[32mINFO\x1b[0m",
        LogLevel::Warning => "\x1b[33mWARN\x1b[0m",
        LogLevel::Error => "\x1b[31mERROR\x1b[0m",
        LogLevel::Critical => "\x1b[35mCRIT\x1b[0m",
      }
    } else {
      match message.level {
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARN",
        LogLevel::Error => "ERROR",
        LogLevel::Critical => "CRIT",
      }
    };

    let thread_name = if self.use_colors {
      format!("\x1b[34m{}\x1b[0m", message.thread_name)
    } else {
      message.thread_name.clone()
    };

    format!(
      "[{}] [{}] [{}] {}: {}",
      time_str, level_str, thread_name, message.source, message.message
    )
  }
}

impl LogOutput for ConsoleOutput {
  fn write(&self, message: &LogMessage) {
    println!("{}", self.format_message(message));
  }
}

pub trait LogFilter: Send + Sync {
  fn allow(&self, message: &LogMessage) -> bool;
}

pub struct LevelFilter {
  min_level: LogLevel,
}

impl LevelFilter {
  pub fn new(min_level: LogLevel) -> Self {
    Self { min_level }
  }
}

impl LogFilter for LevelFilter {
  fn allow(&self, message: &LogMessage) -> bool {
    message.level >= self.min_level
  }
}

pub struct LogCollector {
  receiver: Receiver<LogMessage>,
  outputs: Vec<Box<dyn LogOutput>>,
  filters: Vec<Box<dyn LogFilter>>,
  running: Arc<Mutex<bool>>,
}

impl LogCollector {
  pub fn new(receiver: Receiver<LogMessage>) -> Self {
    Self {
      receiver,
      outputs: Vec::new(),
      filters: Vec::new(),
      running: Arc::new(Mutex::new(false)),
    }
  }

  pub fn add_output(&mut self, output: Box<dyn LogOutput>) {
    self.outputs.push(output);
  }

  pub fn add_filter(&mut self, filter: Box<dyn LogFilter>) {
    self.filters.push(filter);
  }

  pub fn run(&self) {
    if let Ok(mut running) = self.running.lock() {
      *running = true;
    }

    while self.is_running() {
      match self.receiver.recv_timeout(Duration::from_millis(100)) {
        Ok(message) => {
          if self.filters.iter().all(|filter| filter.allow(&message)) {
            for output in &self.outputs {
              output.write(&message);
            }
          }
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
          // Continue loop to check if still running
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
          break;
        }
      }
    }
  }

  pub fn stop(&self) {
    if let Ok(mut running) = self.running.lock() {
      *running = false;
    }
  }

  fn is_running(&self) -> bool {
    self.running.lock().map(|r| *r).unwrap_or(false)
  }
}

pub struct Logger {
  pub identifier: &'static str,
  thread_name: String,
  sender: Sender<LogMessage>,
}

impl Logger {
  pub fn new(identifier: &'static str, thread_name: String, sender: Sender<LogMessage>) -> Self {
    Self {
      identifier,
      thread_name,
      sender,
    }
  }

  pub fn log(&self, level: LogLevel, message: String) {
    let log_message = LogMessage::new(
      self.thread_name.clone(),
      level,
      self.identifier.to_string(),
      message,
    );

    let _ = self.sender.send(log_message);
  }

  pub fn debug<S: Into<String>>(&self, message: S) {
    self.log(LogLevel::Debug, message.into());
  }

  pub fn info<S: Into<String>>(&self, message: S) {
    self.log(LogLevel::Info, message.into());
  }

  pub fn warn<S: Into<String>>(&self, message: S) {
    self.log(LogLevel::Warning, message.into());
  }

  pub fn error<S: Into<String>>(&self, message: S) {
    self.log(LogLevel::Error, message.into());
  }

  pub fn critical<S: Into<String>>(&self, message: S) {
    self.log(LogLevel::Critical, message.into());
  }
}

#[derive(Clone)]
pub struct LoggerSystem {
  sender: Sender<LogMessage>,
}

impl LoggerSystem {
  pub fn new() -> (Self, LogCollector) {
    let (sender, receiver) = mpsc::channel();
    let collector = LogCollector::new(receiver);

    (
      Self {
        sender,
      },
      collector,
    )
  }

  pub fn start_collector(self, collector: LogCollector) -> (Self, thread::JoinHandle<()>) {
    let handle = thread::spawn(move || {
      collector.run();
    });

    (self, handle)
  }

  pub fn create_logger(&self, identifier: &'static str, thread_name: String) -> Logger {
    Logger::new(identifier, thread_name, self.sender.clone())
  }

  pub fn shutdown(self) {
    drop(self.sender);
  }
}

#[macro_export]
macro_rules! log_debug {
  ($logger:expr, $($arg:tt)*) => {
    $logger.debug(format!($($arg)*))
  };
}

#[macro_export]
macro_rules! log_info {
  ($logger:expr, $($arg:tt)*) => {
    $logger.info(format!($($arg)*))
  };
}

#[macro_export]
macro_rules! log_warn {
  ($logger:expr, $($arg:tt)*) => {
    $logger.warn(format!($($arg)*))
  };
}

#[macro_export]
macro_rules! log_error {
  ($logger:expr, $($arg:tt)*) => {
    $logger.error(format!($($arg)*))
  };
}

#[macro_export]
macro_rules! log_critical {
  ($logger:expr, $($arg:tt)*) => {
    $logger.critical(format!($($arg)*))
  };
}
