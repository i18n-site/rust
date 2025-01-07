use std::{thread, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use kanal::bounded;

pub struct Waiting {
  sender: kanal::Sender<Option<String>>,
  handle: Option<thread::JoinHandle<()>>,
}

impl Waiting {
  pub fn new(msg: impl Into<String>) -> Self {
    let (sender, rx) = bounded(1);
    let msg = msg.into();

    let handle = {
      thread::spawn(move || {
        let pb = ProgressBar::new_spinner();
        pb.set_message(msg);
        pb.set_style(
          ProgressStyle::default_spinner()
            .template("{msg} {spinner:.green}")
            .expect("Failed to set template")
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        while let Ok(recv) = rx.try_recv() {
          pb.tick();
          if let Some(recv) = recv {
            if let Some(msg) = recv {
              pb.finish_and_clear();
              println!("{}", msg);
            }
            break;
          } else {
            thread::sleep(Duration::from_millis(100));
          }
        }
      })
    };

    Waiting {
      sender,
      handle: Some(handle),
    }
  }

  pub fn end(self, msg: impl Into<String>) {
    let _ = self.sender.send(Some(msg.into()));
  }
}

impl Drop for Waiting {
  fn drop(&mut self) {
    let _ = self.sender.send(None);
    if let Some(handle) = self.handle.take() {
      let _ = handle.join();
    }
  }
}
