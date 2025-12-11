use std::{env, fs};

use aok::{OK, Void};
use log::{debug, error, info, warn};

// Initialize logger once at startup
#[static_init::constructor(0)]
extern "C" fn _loginit() {
  log_init::init();
}

#[test]
fn test_basic_logging() -> Void {
  info!(">>>>> Basic logging test: {}", 123456);
  warn!("This is a warning message");
  error!("This is an error message");
  debug!("This is a debug message");

  println!("✓ Basic logging test completed");
  OK
}

#[test]
fn test_environment_variable_detection() -> Void {
  println!("Testing environment variable detection...");

  // Check current LOGS_DIRECTORY setting
  match env::var("LOGS_DIRECTORY") {
    Ok(logs_dir) => {
      println!("✓ LOGS_DIRECTORY is set to: {:?}", logs_dir);
      println!("  Logs should be written to files in this directory");

      // Test logging with file output expected
      info!("File logging test message: {}", 789012);
      warn!("File logging warning");

      // Try to find the log file
      let process_name = env::current_exe()
        .ok()
        .and_then(|path| path.file_stem().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or_else(|| "app".to_string());

      let log_file_path = std::path::PathBuf::from(&logs_dir).join(format!("{}.log", process_name));

      // Give some time for the file to be written
      std::thread::sleep(std::time::Duration::from_millis(200));

      if log_file_path.exists() {
        println!("✓ Log file found at: {:?}", log_file_path);
        if let Ok(content) = fs::read_to_string(&log_file_path) {
          let lines: Vec<&str> = content.lines().collect();
          println!("✓ Log file has {} lines", lines.len());
          if !lines.is_empty() {
            println!("✓ Sample log entry: {}", lines[0]);
          }
        }
      } else {
        println!(
          "⚠ Log file not found at: {:?} (may be using stdout fallback)",
          log_file_path
        );
      }
    }
    Err(_) => {
      println!("✓ LOGS_DIRECTORY is not set");
      println!("  Logs should be written to stdout");

      // Test logging with stdout output expected
      info!("Stdout logging test message: {}", 456789);
      warn!("Stdout logging warning");
    }
  }

  println!("✓ Environment variable detection test completed");
  OK
}

#[test]
fn test_manual_file_logging_setup() -> Void {
  println!("Testing manual file logging setup...");

  // Create a temporary directory for this test
  let temp_dir = env::temp_dir().join("log_init_manual_test");
  fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");

  println!("✓ Created test directory: {:?}", temp_dir);
  println!(
    "  To test file logging, set LOGS_DIRECTORY={:?} and restart",
    temp_dir
  );
  println!(
    "  Then run: LOGS_DIRECTORY={:?} cargo test",
    temp_dir.display()
  );

  // Clean up
  let _ = fs::remove_dir_all(&temp_dir);

  OK
}
