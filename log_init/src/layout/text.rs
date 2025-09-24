use std::io::{IsTerminal, stdin};

use colored::{ColoredString, Colorize};

use crate::Kv;

#[derive(Debug)]
pub struct Text {
  pub color: bool,
}

pub fn level_color(level: log::Level) -> ColoredString {
  match level {
    log::Level::Error => level.to_string().red(),
    log::Level::Warn => level.to_string().yellow(),
    log::Level::Info => level.to_string().green(),
    log::Level::Debug => level.to_string().blue(),
    log::Level::Trace => level.to_string().magenta(),
  }
}

impl Default for Text {
  fn default() -> Self {
    Self {
      color: stdin().is_terminal(),
    }
  }
}

impl logforth::Layout for Text {
  fn format(
    &self,
    record: &log::Record,
    diagnostics: &[Box<dyn logforth::diagnostic::Diagnostic>],
  ) -> anyhow::Result<Vec<u8>> {
    let level = record.level();

    let file = if let Some(file) = record.file() {
      if crate::ROOT.len() > 1
        && let Some(f) = file.strip_prefix(crate::ROOT.as_str())
      {
        f.into()
      } else if let Some(f) = file.strip_prefix(crate::HOME_DIR.as_str()) {
        format!("~/{f}")
      } else {
        file.into()
      }
    } else {
      record.target().into()
    };

    let file_line = if let Some(line) = record.line() {
      format!("{file}:{line}")
    } else {
      file
    };

    let msg = record.args();

    let mut visitor = Kv {
      text: String::new(),
    };
    record.key_values().visit(&mut visitor)?;
    for d in diagnostics {
      d.visit(&mut visitor)?;
    }
    let msg = if visitor.text.is_empty() {
      msg.to_string()
    } else {
      format!("{} {}", msg, visitor.text)
    };

    let msg = if self.color {
      let level = level_color(level);
      let file_line = file_line.bright_black();
      format!("{level} {file_line} {msg}")
    } else {
      let ts = coarsetime::Clock::now_since_epoch().as_secs();
      let ts = {
        if let Ok(timestamp) = jiff::Timestamp::from_second(ts as i64) {
          jiff::Zoned::new(timestamp, crate::TZ.clone())
            .strftime("%Y-%m-%d %H:%M:%S")
            .to_string()
        } else {
          ts.to_string()
        }
      };
      format!("{level} {ts} {file_line} {msg}")
    };

    Ok(msg.as_bytes().into())
  }
}
