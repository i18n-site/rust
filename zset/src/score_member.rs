use std::cmp::Ordering;

use crate::{ArcM, Key, Member, Score};

pub(crate) struct ScoreMember<K: Key, M: Member<K>, S> {
  pub(crate) score: S,
  pub(crate) member: ArcM<K, M>,
}

impl<K: Key, M: Member<K>, S: Clone> Clone for ScoreMember<K, M, S> {
  fn clone(&self) -> Self {
    Self {
      score: self.score.clone(),
      member: self.member.clone(),
    }
  }
}

impl<K: Key, M: Member<K>, S: Score> PartialEq for ScoreMember<K, M, S> {
  fn eq(&self, other: &Self) -> bool {
    self.score.eq(&other.score) && self.member.borrow().eq(other.member.borrow())
  }
}

impl<K: Key, M: Member<K>, S: Score> Eq for ScoreMember<K, M, S> {}

impl<K: Key, M: Member<K>, S: Score> PartialOrd for ScoreMember<K, M, S> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<K: Key, M: Member<K>, S: Score> Ord for ScoreMember<K, M, S> {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .score
      .cmp(&other.score)
      .then_with(|| self.member.borrow().cmp(other.member.borrow()))
  }
}
