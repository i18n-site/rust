use roaring::RoaringTreemap;

pub fn merge<S: AsRef<str>>(li: impl IntoIterator<Item = S>, skip_li: &RoaringTreemap) -> Vec<u8> {
  let mut txt = vec![];
  for (pos, s) in li.into_iter().enumerate() {
    if skip_li.contains(pos as u64) {
      txt.push(0);
    } else {
      txt.extend(s.as_ref().as_bytes());
    }
  }
  txt
}

pub fn split<T: AsRef<[u8]>>(bin: T) -> Vec<String> {
  let bin = bin.as_ref();
  let mut li = vec![];
  let mut pre = 0;

  for (i, byte) in bin.iter().enumerate() {
    if *byte == 0 {
      if pre < i {
        li.push(String::from_utf8_lossy(&bin[pre..i]).into());
      }
      li.push(String::new());
      pre = i + 1;
    }
  }

  if pre < bin.len() {
    li.push(String::from_utf8_lossy(&bin[pre..]).into());
  }

  li
}
