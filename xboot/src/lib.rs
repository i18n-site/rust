use linkme::distributed_slice;

#[distributed_slice]
pub static BOOT: [fn()];

pub fn init() {
  for i in BOOT {
    i()
  }
}
