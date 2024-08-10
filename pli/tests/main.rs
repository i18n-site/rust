use aok::Result;
use pli::Pli;
use static_init::constructor;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let mut txt_li = vec![
    "0 hello".to_string(),
    "1 world".to_string(),
    "2 rust".to_string(),
  ];
  let pos_li = vec![2, 0, 1];

  let mut li_proxy = Pli::new(&mut txt_li, pos_li);

  // 迭代
  for item in li_proxy.iter() {
    println!("{}", item);
  }
  // 访问和修改
  println!("{}", li_proxy[0]); // 输出 "rust"
  li_proxy[2] = "<".to_string();
  println!("{}", li_proxy[0]); // 输出 "Rust"

  {
    for (pos, mut i) in li_proxy.iter_mut().enumerate() {
      *i = "xx".into();
      if pos == 1 {
        break;
      }
    }
  }
  {
    dbg!(&txt_li);
  }
  Ok(())
}
