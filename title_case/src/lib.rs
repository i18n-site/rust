use ucfirst::ucfirst;

pub fn title_case<'a>(now: impl IntoIterator<Item = &'a str>, pre: &str) -> String {
  now
    .into_iter()
    .map(|i| {
      // i18n 是什么？-> What Is i18n ?

      let mut t = String::new();
      for j in i.chars() {
        if "?!.;".contains(j) {
          break;
        }
        t.push(j);
      }

      if pre.contains(&t) {
        i.into()
      } else {
        ucfirst(i)
      }
    })
    .collect::<Vec<_>>()
    .join(" ")
}
