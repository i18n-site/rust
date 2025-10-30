use aok::{OK, Void};
use ext_name::ext_name;
use log::info;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

#[test]
fn test() -> Void {
  for i in ["1.mp3", ".mp3", "1.", "", "."] {
    info!("{i} {:?}", ext_name(i));
  }
  OK
}
