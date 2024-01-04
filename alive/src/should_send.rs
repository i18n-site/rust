pub fn is_power_of_two(n: u32) -> bool {
  (n > 0) && ((n & (n - 1)) == 0)
}

pub fn should_send(err_count: u32, warn_err: u8) -> bool {
  let warn_err = warn_err as _;
  if err_count >= warn_err {
    let diff = err_count - warn_err;
    if diff > 1440 {
      diff % 1439 == 0
    } else {
      // 1=4(W) 2=5 3=6 4=7 5=8(W)
      is_power_of_two(diff + 4)
    }
  } else {
    false
  }
}
