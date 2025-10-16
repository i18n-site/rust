use unicode_script::{Script, UnicodeScript};

pub fn state(c: char) -> State {
  if c.is_whitespace() {
    return State::Space;
  }
  if matches!(
    c.script(),
    Script::Han
      | Script::Hiragana
      | Script::Katakana
      | Script::Thai
      | Script::Lao
      | Script::Khmer
      | Script::Myanmar
      | Script::Tibetan
  ) || ('０'..='９').contains(&c)
  {
    return State::Char;
  }

  if c == '`'{
    return State::Letter;
  }

  if r##"!"#%\'*+,-.:<=>?@^·—‘’“”…、。「」『』！，：？；（）"##.contains(c)
    || (c.len_utf8() > 1 && unic_emoji_char::is_emoji(c))
  {
    return State::Punctuation;
  }

  State::Letter
}

fn push_stack(c: char, stack: &mut Vec<char>) {
  if "[({".contains(c) {
    stack.push(c);
  }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum State {
  Space,
  Char,
  Letter,
  Punctuation,
}

pub fn add_space(txt: impl AsRef<str>) -> String {
  let txt = txt.as_ref();
  let mut r = String::new();
  let mut iter = txt.chars();

  if let Some(c) = iter.next() {
    r.push(c);
    let mut is_escape = c == '\\';
    let mut pre = state(c);
    let mut pre_c = c;
    let mut stack = Vec::new();

    push_stack(c, &mut stack);
    for c in iter {
      if is_escape {
        is_escape = false;
        r.push(c);
        continue;
      }
      let s = state(c);
      push_stack(c, &mut stack);
      match s {
        State::Char => {
          if pre == State::Letter && !"[({".contains(pre_c) {
            r.push(' ');
          }
          r.push(c);
        }
        State::Letter => {
          is_escape = c == '\\';
          if let Some(stack_last) = stack.last() {
            if matches!((stack_last, c), ('[', ']') | ('(', ')') | ('{', '}')) {
              stack.pop();
            }
          } else if (!is_escape && pre == State::Char)
            || (",?!…".contains(pre_c))
            || (pre_c == '.' && c.is_uppercase())
          {
            r.push(' ');
          }
          r.push(c);
        }
        _ => r.push(c),
      }
      pre = s;
      pre_c = c;
    }
  }
  r
}
