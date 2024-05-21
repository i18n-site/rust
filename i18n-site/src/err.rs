use thiserror::Error;

#[derive(Error, Debug)]
pub enum Err {
  // #[error("{0} : {1}")]
  // Conf(PathBuf, serde_yaml::Error),
}
