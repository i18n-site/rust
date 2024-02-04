pub const URL_ID: &[u8] =
  b"!#$&'()*+,-./0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz~";

pub struct Rany<'a>(pub &'a [u8]);

impl<'a> Rany<'a> {
  pub fn dbin(&self, s: &[u8]) -> usize {
    let alphabet = self.0;
    let len = alphabet.len();
    let mut result = 0;
    for (i, c) in s.iter().rev().enumerate() {
      let digit = alphabet
        .iter()
        .position(|x| x == c)
        .expect("invalid string");
      result += digit * len.pow(i as _);
    }

    result
  }

  pub fn d(&self, s: impl AsRef<str>) -> usize {
    self.dbin(s.as_ref().as_bytes())
  }

  pub fn ebin(&self, mut num: usize) -> Vec<u8> {
    let alphabet = self.0;
    if num == 0 {
      vec![alphabet[0]]
    } else {
      let len = alphabet.len();
      let mut result = Vec::new();

      while num != 0 {
        let remainder = num % len;
        result.push(alphabet[remainder]);
        num /= len;
      }

      result.into_iter().rev().collect()
    }
  }

  pub fn e(&self, num: usize) -> String {
    String::from_utf8(self.ebin(num)).unwrap()
  }
}

pub const RANY_URL_ID: Rany = Rany(URL_ID);
