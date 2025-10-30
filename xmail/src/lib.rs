use xstr::cut255 as cut;

pub fn user_host(mail: impl AsRef<str>) -> (String, String) {
  let mail = mail.as_ref();
  let user;

  if let Some(p) = mail.find('@') {
    user = cut(&mail[..p]).to_owned();
    if mail.len() > p {
      return (user, cut(&mail[p + 1..]).into());
    }
  } else {
    user = "".to_owned();
  }

  (user, cut(mail).into())
}

pub fn norm_user_host(mail: impl AsRef<str>) -> (String, String) {
  let mail = xstr::lowtrim(mail);
  let (user, host) = user_host(mail);
  let host = host
    .split('.')
    .map(|i| {
      let i = i.trim();
      if i.starts_with("xn--")
        && let Ok(i) = punycode::decode(&i[3..])
      {
        i
      } else {
        i.to_owned()
      }
    })
    .collect::<Vec<_>>()
    .join(".");
  (user, host)
}

pub fn norm(mail: impl AsRef<str>) -> String {
  let (user, host) = norm_user_host(mail);
  user + "@" + &host
}

pub fn norm_tld(mail: impl AsRef<str>) -> (String, String) {
  let (user, host) = norm_user_host(mail);
  (user + "@" + &host, xtld::tld(&host).into())
}
