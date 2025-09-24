#[derive(Debug, Default, Clone)]
pub struct LiPos(pub Vec<String>);

impl LiPos {
  pub fn id(&mut self, ver: impl Into<String>) -> usize {
    let ver = ver.into();
    if let Some(i) = self.0.iter().position(|v| *v == ver) {
      return i;
    }
    let i = self.0.len();
    self.0.push(ver);
    i
  }
}
