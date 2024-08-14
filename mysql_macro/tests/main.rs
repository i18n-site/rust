use mysql_macro::q1;

#[tokio::test]
async fn main() -> aok::Result<()> {
  loginit::init();

  let sql = format!("SELECT {}", 1);
  let r: u64 = q1!(sql);
  // e(sql.clone(), vec![]).await?;

  // tracing::debug!("test");
  // let id_li = vec![1];
  // let li: HashMap<_, String> = id_v("payBrand", id_li).await?;
  // let li: HashMap<_, String> = id_v("payBrand", *&[1]).await?;

  // let mail_id: Option<Option<u64>> = q01!(r#"select mailId("a@b.c")"#);
  // let mail_id: Option<u64> = q1!(r#"select mailId("a@b.c")"#);
  //
  // e!(r"select mailHostid('a.com')");
  //
  // let mut conn = conn!();
  //
  // let mail_host_id: u64 = q1!(&mut conn; r"select mailHostid('a.com')");
  // let mail_host_id: u64 = q1!(r"select mailHostid('a.com')");
  // let mail_host_id: u64 = q1!(r"select mailHostid(?)", "a.com");
  // let mail_host_id: Option<u64> = q01!("select mailHostid('a.com')",);
  //
  // let q: Vec<Option<u64>> = q!(&mut conn; r"select mailHostid(?)","a.com");
  //
  // let q: Vec<(u64,)> = q!(r"select mailHostid(?)", "a.com");
  //
  // let q: Vec<u64> = q!(r"select mailHostid(?)", "a.com");

  // let s = r#"'\'test\''"#;
  // println!("{}", mysql_macro::s(s));
  //
  // let s = [211, 222, 223, 224, 225];
  // println!("{}", mysql_macro::b(&s[..]));
  Ok(())
}
