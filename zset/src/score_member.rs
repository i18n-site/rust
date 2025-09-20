use std::{cmp::Ordering, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) struct ScoreMember<M, S> {
  pub(crate) score: S,
  pub(crate) member: Arc<M>,
}

impl<M: Ord, S: Ord> PartialEq for ScoreMember<M, S> {
  fn eq(&self, other: &Self) -> bool {
    self.score.eq(&other.score) && self.member.as_ref().eq(other.member.as_ref())
  }
}

impl<M: Ord, S: Ord> Eq for ScoreMember<M, S> {}

impl<M: Ord, S: Ord> PartialOrd for ScoreMember<M, S> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<M: Ord, S: Ord> Ord for ScoreMember<M, S> {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .score
      .cmp(&other.score)
      .then_with(|| self.member.as_ref().cmp(other.member.as_ref()))
  }
}
