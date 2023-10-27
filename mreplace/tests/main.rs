#[cfg(test)]
mod tests {
  use lazy_static::lazy_static;
  use mreplace::Mreplace;

  lazy_static! {
    static ref RENDER: Mreplace = Mreplace::new(["${host}", "${action}", "${code}"]).unwrap();
  }

  #[test]
  fn test_replace() {
    let txt = RENDER.replace(
      "${host} ${action} Code: ${code} .",
      ["3Ti.Site", "SignUp", "XV1z"],
    );
    assert_eq!(txt, "3Ti.Site SignUp Code: XV1z .");
  }
}
