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
