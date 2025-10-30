use txt_li::TxtLi;

pub fn i18n(txt_li: &mut TxtLi, code: &str) {
  if code.is_empty() {
    return;
  }

  for line in code.lines() {
    #[allow(clippy::never_loop)]
    'out: loop {
      let trim = line.trim_start();
      for i in ["#", "//"] {
        if trim.starts_with(i) {
          let indent = line.len() - trim.len();
          let start = indent + i.len();
          if start > 0 {
            txt_li.push_no_tran(&line[..start]);
          }
          txt_li.push_md(&line[start..]);
          break 'out;
        }
      }

      #[allow(clippy::never_loop)]
      'o1: loop {
        let mut pre = 0;
        let mut iter = line.char_indices();

        while let Some((pos, i)) = iter.next() {
          if i.is_ascii() || upn::is_full_width_break(i) {
            continue;
          }
          if pos != pre {
            txt_li.push_no_tran(&line[pre..pos]);
          }

          pre = pos;
          #[allow(clippy::never_loop)]
          'o2: loop {
            for (pos, i) in iter.by_ref() {
              if "<>{}):：]".contains(i) {
                txt_li.push_md(&line[pre..pos]);
                pre = pos;
                break 'o2;
              }
            }
            txt_li.push_md(&line[pre..]);
            break 'o1;
          }
        }
        if pre < line.len() {
          txt_li.push_no_tran(&line[pre..]);
        }
        break;
      }

      break;
    }
    txt_li.push_no_tran("\n");
  }
  txt_li.restore.trim_last();
}
