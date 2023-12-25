#![feature(let_chains)]
use xstr::truncate255 as truncate;

pub fn user_host(mail: impl AsRef<str>) -> (String, String) {
  let mail = mail.as_ref();
  let user;

  if let Some(p) = mail.find('@') {
    user = truncate(mail[..p].to_owned());
    if mail.len() > p {
      return (user, truncate(&mail[p + 1..]));
    }
  } else {
    user = "".to_owned();
  }

  (user, truncate(mail))
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
  (user + "@" + &host, xtld::host_tld(host))
}
