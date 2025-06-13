pub fn ignore(c: char) -> bool {
  c.is_whitespace() || "`!?.;。".contains(c)
}

pub fn split(input: &str) -> Vec<String> {
  let mut result = Vec::new();
  let mut word = String::new();
  let mut whitespace = String::new();

  for c in input.chars() {
    if ignore(c) {
      if !word.is_empty() {
        result.push(word.clone());
        word.clear();
      }
      whitespace.push(c);
    } else {
      if !whitespace.is_empty() {
        result.push(whitespace.clone());
        whitespace.clear();
      }
      word.push(c);
    }
  }

  if !word.is_empty() {
    result.push(word);
  } else if !whitespace.is_empty() {
    result.push(whitespace);
  }

  result
}

pub fn title_case(now: impl AsRef<str>, pre: impl AsRef<str>) -> String {
  let now = now.as_ref();
  let pre = pre.as_ref();
  let mut case = std::collections::HashMap::new();
  for i in split(pre) {
    if !ignore(i.chars().next().unwrap()) {
      case.insert(i.to_lowercase(), i);
    }
  }

  let now = titlecase::titlecase(now);
  let li = split(&now);
  let mut t = Vec::with_capacity(li.len());
  for i in li {
    if ignore(i.chars().next().unwrap()) {
      t.push(i)
    } else if let Some(i) = case.get(&i.to_lowercase()) {
      t.push(i.clone());
    } else {
      t.push(i);
    }
  }

  t.join("")
}
