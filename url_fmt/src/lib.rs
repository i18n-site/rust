use percent_encoding::percent_decode_str;

pub fn url_fmt(url: impl AsRef<str>) -> String {
  let url = url.as_ref();
  let hidden_url = 'block: {
    if let Some((protocol, rest)) = url.split_once("://")
      && let Some(at_index) = rest.rfind('@')
    {
      let host_part = &rest[at_index + 1..];
      break 'block format!("{}://*@{}", protocol, host_part);
    }
    url.to_string()
  };

  if let Some(hash_index) = hidden_url.find('#') {
    let (base, fragment_with_hash) = hidden_url.split_at(hash_index);
    if let Ok(decoded_fragment) = percent_decode_str(&fragment_with_hash[1..]).decode_utf8() {
      return format!("{}#{}", base, decoded_fragment);
    }
  }

  hidden_url
}
