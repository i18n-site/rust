use crate::TxtPos;

pub fn i18n<'a>(code: &'a str, txtpos: &mut TxtPos<'a>) {
  if code.is_empty() {
    return;
  }

  let code = code.split('\n');

  for line in code {
    #[allow(clippy::never_loop)]
    'out: loop {
      let trim = line.trim_start();
      for i in ["#", "//"] {
        if trim.starts_with(i) {
          let indent = line.len() - trim.len();
          let start = indent + i.len();
          if start > 0 {
            txtpos.txt_li.push(&line[..start]);
          }
          txtpos.push_txt_line(&line[start..]);
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
            txtpos.txt_li.push(&line[pre..pos]);
          }

          pre = pos;
          #[allow(clippy::never_loop)]
          'o2: loop {
            while let Some((pos, i)) = iter.next() {
              if "<>{}):：]".contains(i) {
                txtpos.push_pos(&line[pre..pos]);
                pre = pos;
                break 'o2;
              }
            }
            txtpos.push_pos(&line[pre..]);
            break 'o1;
          }
        }
        if pre < line.len() {
          txtpos.txt_li.push(&line[pre..]);
        }
        break;
      }

      break;
    }
    txtpos.txt_li.push("\n");
  }
  txtpos.txt_li.pop();
}
