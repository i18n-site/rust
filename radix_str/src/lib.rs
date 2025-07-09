use proc_macro::TokenStream;

#[proc_macro]
pub fn radix_str(input: TokenStream) -> proc_macro::TokenStream {
  let mut n_radix: [String; 2] = [String::new(), String::new()];
  for (p, s) in input.to_string().replace(' ', "").split(',').enumerate() {
    if p == 0 {
      n_radix[p] = s.into();
    } else {
      n_radix[p] = s.into();
      break;
    }
  }
  let mut n = &n_radix[0][..];
  let prefix = if n.starts_with('-') {
    n = &n[1..];
    "-"
  } else {
    ""
  };
  let radix: u8 = n_radix[1].parse().unwrap();
  let n: u64 = n.parse().unwrap();
  let out = "\"".to_owned() + prefix + &radix_fmt::radix(n, radix).to_string() + "\"";

  out.parse().unwrap()
}
