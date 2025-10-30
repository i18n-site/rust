pub fn text_url(s: impl AsRef<str>) -> bool {
  let s = s.as_ref();
  let mut has_dot = false;
  let mut has_left_paren = false;

  for c in s.chars() {
    if !c.is_ascii() || c.is_whitespace() {
      return false;
    }
    match c {
      '.' => has_dot = true,
      '(' => has_left_paren = true,
      ')' => {
        if has_left_paren {
          return false;
        }
      }
      _ => {}
    }
  }

  has_dot
}

pub fn only_url(input: impl AsRef<str>) -> bool {
  let input = input.as_ref();
  if text_url(input) {
    return true;
  }
  enum State {
    Normal,
    InBrackets,
    InParentheses,
  }

  let mut state = State::Normal;
  let mut iter = input.char_indices().peekable();
  let mut bracket_begin = 0;

  while let Some((pos, c)) = iter.next() {
    match state {
      State::Normal => {
        if c == '[' {
          bracket_begin = pos + 1;
          state = State::InBrackets;
        } else if "/!".contains(c) || c.is_whitespace() {
          continue; // Ignore ! and whitespace
        } else {
          return false; // Encounter non-whitespace character
        }
      }
      State::InBrackets => {
        if c == ']' {
          let name = input[bracket_begin..pos].trim();
          if !name.is_empty() {
            return false;
          }
          if let Some((_, '(')) = iter.peek() {
            iter.next(); // Move past '('
            state = State::InParentheses;
          } else {
            state = State::Normal;
          }
        }
      }
      State::InParentheses => {
        if c == ')' {
          state = State::Normal;
        }
      }
    }
  }

  true // All characters processed and no non-whitespace found
}
