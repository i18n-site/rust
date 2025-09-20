use std::{hash::Hash, ops::Range, sync::Arc};

#[cfg(feature = "impl")]
pub mod score_member;
#[cfg(feature = "impl")]
pub mod zset_impl;
#[cfg(feature = "impl")]
pub use zset_impl::Zset;

/// The public API for the Zset.
/// Zset 的公共 API。
pub trait Api<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static,
  S: Ord + Send + Sync + 'static + Clone,
{
  /// Adds the specified member with the specified score to the sorted set.
  /// If the member is already a member of the sorted set, the score is updated.
  /// Returns `true` if the score was updated, `false` if a new member was added.
  /// 将指定的成员和分数添加到排序集合中。
  /// 如果成员已经是排序集合的成员，则更新其分数。
  /// 如果分数被更新，则返回 `true`；如果是新成员，则返回 `false`。
  fn add(&self, member: M, score: S) -> bool;

  /// Removes the specified member from the sorted set.
  /// 从排序集合中移除指定的成员。
  fn remove(&self, member: &M) -> bool;

  /// Returns the score of the specified member.
  /// 返回指定成员的分数。
  fn score(&self, member: &M) -> Option<S>;

  /// Returns the 0-based rank of the member in the sorted set, with scores ordered from low to high.
  /// 返回成员在排序集合中的排名（0-based），分数从低到高排序。
  fn rank(&self, member: &M) -> Option<usize>;

  /// Returns the number of elements in the sorted set (cardinality).
  /// 返回排序集合中的元素数量（基数）。
  fn card(&self) -> usize;

  /// Returns the number of elements in the sorted set. This is an alias for `card()`.
  /// 返回排序集合中的元素数量。这是 `card()` 的别名。
  fn len(&self) -> usize {
    self.card()
  }

  /// Returns `true` if the sorted set contains no elements.
  /// 如果排序集合为空，则返回 `true`。
  fn is_empty(&self) -> bool;

  /// Returns a range of members in the sorted set, with scores ordered from low to high.
  /// The `range` is a 0-based, half-open interval (`start..end`).
  /// 返回排序集合中指定范围的成员，分数从低到高排序。
  /// `range` 是一个 0-based 的半开区间 (`start..end`)。
  fn range(&self, range: Range<usize>) -> Vec<Arc<M>>;

  /// Returns a range of members with their scores in the sorted set, with scores ordered from low to high.
  /// The `range` is a 0-based, half-open interval (`start..end`).
  /// 返回排序集合中指定范围的成员及其分数，分数从低到高排序。
  /// `range` 是一个 0-based 的半开区间 (`start..end`)。
  fn range_with_scores(&self, range: Range<usize>) -> Vec<(Arc<M>, S)>;

  /// Returns the member at the specified 0-based rank, with scores ordered from low to high.
  /// 返回指定排名（0-based）的成员，分数从低到高排序。
  fn get(&self, rank: usize) -> Option<Arc<M>>;

  /// Returns the member and score at the specified 0-based rank, with scores ordered from low to high.
  /// 返回指定排名（0-based）的成员和分数，分数从低到高排序。
  fn get_with_score(&self, rank: usize) -> Option<(Arc<M>, S)>;
}
