use std::{borrow::Borrow, hash::Hash, ops::RangeBounds, sync::Arc};
mod arc_m;
pub use arc_m::ArcM;

#[cfg(feature = "impl")]
pub mod score_member;
#[cfg(feature = "impl")]
pub mod zset_impl;
#[cfg(feature = "impl")]
pub use zset_impl::Zset;

pub trait Score: Ord + Clone {}

impl<T: Ord + Clone> Score for T {}

pub trait Key: Eq + Hash + Ord + Clone {}

impl<T: Eq + Hash + Ord + Clone> Key for T {}

pub trait Member<K: Key>: Borrow<K> {}

impl<T, K: Key> Member<K> for T where T: Borrow<K> {}

/// The public API for the Zset.
/// Zset 的公共 API。
pub trait Api<M, S>
where
  M: Borrow<Self::K>,
  S: Score,
{
  type K: Key;
  type Item;
  /// Adds the specified member with the specified score to the sorted set.
  /// If the member is already a member of the sorted set, the score is updated.
  /// Returns `true` if the score was updated, `false` if a new member was added.
  /// 将指定的成员和分数添加到排序集合中。
  /// 如果成员已经是排序集合的成员，则更新其分数。
  /// 如果分数被更新，则返回 `true`；如果是新成员，则返回 `false`。
  fn add(&self, member: impl Into<Self::Item>, score: S) -> bool;

  /// Removes the specified member from the sorted set.
  /// 从排序集合中移除指定的成员。
  fn rm(&self, member: impl Borrow<Self::K>) -> bool;

  /// Removes a range of members in the sorted set, with scores ordered from low to high.
  /// The `range` is a 0-based.
  /// Returns the number of members removed.
  /// 按排名（0-based）从低到高，删除排序集合中指定范围的成员。
  /// 返回被删除的成员数量。
  fn rm_range_by_rank(&self, range: impl RangeBounds<usize>) -> usize;

  /// Returns the score of the specified member.
  /// 返回指定成员的分数。
  fn score(&self, member: impl Borrow<Self::K>) -> Option<S>;

  /// Returns the 0-based rank of the member in the sorted set, with scores ordered from low to high.
  /// 返回成员在排序集合中的排名（0-based），分数从低到高排序。
  fn rank(&self, member: impl Borrow<Self::K>) -> Option<usize>;

  /// Returns the number of elements in the sorted set (leninality).
  /// 返回排序集合中的元素数量（基数）。
  fn len(&self) -> usize;

  /// Returns the number of elements in the sorted set. This is an alias for `len()`.
  /// 返回排序集合中的元素数量。这是 `len()` 的别名。
  #[cfg(feature = "card")]
  fn card(&self) -> usize {
    self.len()
  }

  /// Returns `true` if the sorted set contains no elements.
  /// 如果排序集合为空，则返回 `true`。
  fn is_empty(&self) -> bool;

  /// Returns `true` if the sorted set contains the specified member.
  /// 如果排序集合中包含指定的成员，则返回 `true`。
  fn contains(&self, member: impl Borrow<Self::K>) -> bool;

  /// Returns a range of members in the sorted set, with scores ordered from low to high.
  /// The `range` is a 0-based, half-open interval (`start..end`).
  /// 返回排序集合中指定范围的成员，分数从低到高排序。
  /// `range` 是一个 0-based 的半开区间 (`start..end`)。
  fn range(&self, range: impl RangeBounds<usize>) -> Vec<Arc<M>>;

  /// Returns a range of members with their scores in the sorted set, with scores ordered from low to high.
  /// The `range` is a 0-based, half-open interval (`start..end`).
  /// 返回排序集合中指定范围的成员及其分数，分数从低到高排序。
  /// `range` 是一个 0-based 的半开区间 (`start..end`)。
  fn range_with_scores(&self, range: impl RangeBounds<usize>) -> Vec<(Arc<M>, S)>;

  /// Returns the member at the specified 0-based rank, with scores ordered from low to high.
  /// 返回指定排名（0-based）的成员，分数从低到高排序。
  fn get(&self, rank: usize) -> Option<Arc<M>>;

  /// Returns the member and score at the specified 0-based rank, with scores ordered from low to high.
  /// 返回指定排名（0-based）的成员和分数，分数从低到高排序。
  fn get_with_score(&self, rank: usize) -> Option<(Arc<M>, S)>;
}
