use aok::Result;
use idoh::Answer;
use ver_from_txt::{VerUrlLi, ver_from_txt};

fn extract(project: &str, pre_ver: &[u64; 3], li: Vec<Answer>) -> Result<Option<Option<VerUrlLi>>> {
  for i in li {
    if i.r#type == idoh::record_type::TXT {
      return Ok(Some(ver_from_txt(project, pre_ver, &i.data)?));
    }
  }
  Ok(None)
}

pub async fn dns_check(
  project: &str,
  pre_ver: &[u64; 3],
  txt_host_li: &[String],
) -> Result<Option<VerUrlLi>> {
  for host in riter::iter(txt_host_li) {
    let pre_ver = *pre_ver;
    let project = project.to_owned();
    if let Ok(r) =
      xerr::ok!(idoh::resolve(host, "TXT", move |li| extract(&project, &pre_ver, li)).await)
    {
      return Ok(r);
    }
  }
  Ok(None)
}
