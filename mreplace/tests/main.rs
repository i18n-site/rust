use mreplace::Mreplace;

#[static_init::dynamic]
static RENDER: Mreplace = Mreplace::new(["${host}", "${action}", "${code}"]).unwrap();

#[test]
fn test_replace() {
  let txt = "${host} ${action} Code: ${code} .";
  let args = ["3Ti.Site", "SignUp", "XV1z"];
  let result = "3Ti.Site SignUp Code: XV1z .";
  assert_eq!(RENDER.replace(txt, args), result);
}

#[cfg(feature = "macro")]
mod test_macro {
  use mreplace::mreplace;
  mreplace!(
      RENDER2: host action code;
      RENDER3: host action code
  );
  #[test]
  fn test_replace() {
    let txt = "${host} ${action} Code: ${code} .";
    let args = ["3Ti.Site", "SignUp", "XV1z"];
    let result = "3Ti.Site SignUp Code: XV1z .";
    assert_eq!(RENDER2.replace(txt, args), result);
    assert_eq!(RENDER3.replace(txt, args), result);
  }
}
