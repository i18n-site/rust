use std::env;

use aok::{OK, Void};
use log::{debug, error, info, warn};

// Initialize logger once at startup
#[static_init::constructor(0)]
extern "C" fn _loginit() {
  log_init::init();
}

#[test]
fn test_stdout_logging() -> Void {
  info!(">>>>> Stdout logging test: {}", 123456);
  warn!("This is a warning message");
  error!("This is an error message");
  debug!("This is a debug message");

  println!("✓ Stdout logging test completed");
  OK
}

#[test]
fn test_journald_detection() -> Void {
  println!("Testing journald detection...");

  match env::var("INVOCATION_ID") {
    Ok(invocation_id) => {
      println!("✓ INVOCATION_ID found: {}", invocation_id);
      println!("  Running in systemd environment - journald logging should be active");

      // Test logging that should go to journald
      info!("Journald logging test message: {}", 789012);
      warn!("Journald logging warning");
    }
    Err(_) => {
      println!("✓ INVOCATION_ID not found");
      println!("  Not running in systemd environment - stdout logging should be active");

      // Test logging that should go to stdout
      info!("Stdout fallback logging test message: {}", 456789);
      warn!("Stdout fallback logging warning");
    }
  }

  println!("✓ Journald detection test completed");
  OK
}
