use std::{thread, time::Duration};

use aok::{OK, Result};
use waiting::Waiting;

#[test]
fn test() -> Result<()> {
  let ing = Waiting::new("refresh version");
  thread::sleep(Duration::from_secs(3));
  ing.end("✅ task done");
  OK
}
