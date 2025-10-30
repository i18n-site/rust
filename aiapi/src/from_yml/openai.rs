use saphyr::{LoadableYamlNode, Yaml};

use crate::{Error, Result, TokenLi, openai::OpenAI};

pub fn loads(yml: impl AsRef<str>) -> Result<TokenLi<OpenAI>> {
  let yml = yml.as_ref();
  let yml = Yaml::load_from_str(yml)?;
  if yml.is_empty() {
    return Err(Error::ConfTrait("yml format error".to_string()));
  }
  let yml = &yml[0];
  let url = yml["url"]
    .as_str()
    .ok_or(Error::ConfTrait("url".to_string()))?;
  let concurrent = yml["concurrent"].as_integer().unwrap_or(1);
  let token_li = yml["token_li"]
    .as_vec()
    .ok_or(Error::ConfTrait("token_li must be an array".to_string()))?
    .iter()
    .map(|v| v.as_str())
    .collect::<Option<Vec<_>>>()
    .ok_or(Error::ConfTrait(
      "token_li must be an array of strings".to_string(),
    ))?;

  Ok(TokenLi::new(
    token_li,
    concurrent as usize,
    OpenAI::new(url.to_string()),
  ))
}

pub fn load(path: impl AsRef<std::path::Path>) -> Result<TokenLi<OpenAI>> {
  let path = path.as_ref();
  let content = std::fs::read_to_string(path).map_err(|error| Error::File {
    error,
    path: path.display().to_string(),
  })?;
  loads(&content)
}
