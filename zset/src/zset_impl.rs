use std::{
  borrow::Borrow,
  ops::{Range, RangeBounds},
  sync::Arc,
};

use dashmap::DashMap;
use parking_lot::RwLock;
use skiplist::ordered_skiplist::OrderedSkipList;

use crate::{Api, Key, Member, Score, score_member::ScoreMember};

/// A thread-safe, Redis-like sorted set implementation.
/// 一个线程安全的、类似 Redis 的排序集合实现。
pub struct Zset<K, M, S>
where
  M: Member<K>,
  K: Key,
  S: Score,
{
  map: DashMap<K, ScoreMember<K, M, S>>,
  list: RwLock<OrderedSkipList<ScoreMember<K, M, S>>>,
}

fn resolve_range(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
  let start = match range.start_bound() {
    std::ops::Bound::Included(&s) => s,
    std::ops::Bound::Excluded(&s) => s + 1,
    std::ops::Bound::Unbounded => 0,
  };
  let end = match range.end_bound() {
    std::ops::Bound::Included(&e) => e + 1,
    std::ops::Bound::Excluded(&e) => e,
    std::ops::Bound::Unbounded => len,
  };
  start.min(len)..end.min(len)
}

macro_rules! with_range_iter {
  ($self:expr, $range:expr, $closure:expr) => {{
    let list = $self.list.read();
    let len = list.len();
    let range = resolve_range($range, len);
    let iter = list.index_range(range);
    $closure(iter)
  }};
}

impl<K, M, S> Zset<K, M, S>
where
  K: Key,
  M: Member<K>,
  S: Score,
{
  /// Creates a new, empty Zset.
  /// 创建一个新的空 Zset。
  pub fn new() -> Self {
    Self {
      map: DashMap::new(),
      list: RwLock::new(OrderedSkipList::new()),
    }
  }

  fn with_item_by_rank<F, R>(&self, rank: usize, f: F) -> Option<R>
  where
    F: FnOnce(&ScoreMember<K, M, S>) -> R,
  {
    self.list.read().get(rank).map(f)
  }
}

impl<K, M, S> Default for Zset<K, M, S>
where
  K: Key,
  M: Member<K>,
  S: Score,
{
  /// Creates a new, empty Zset.
  /// 创建一个新的空 Zset。
  fn default() -> Self {
    Self::new()
  }
}

impl<K, M, S> Api<M, S> for Zset<K, M, S>
where
  K: Key,
  M: Member<K>,
  S: Score,
{
  type K = K;
  type Item = crate::ArcM<K, M>;

  /// Adds a member with a score.
  /// If the member already exists, its score is updated.
  /// 添加一个成员及其分数。
  /// 如果成员已存在，则更新其分数。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(log N)
  fn add(&self, member: impl Into<Self::Item>, score: S) -> bool {
    let member = member.into();
    let key: &Self::K = (*member).borrow();
    let key = key.clone();
    let mut list = self.list.write();
    {
      if let Some(mut item) = self.map.get_mut(&key) {
        if item.score == score {
          return false; // Score is the same, no update needed
        }
        // O(log N) removal
        list.remove(&*item);
        // Now update the score in the map and insert the new ScoreMember
        item.score = score.clone();
        let new_item = item.clone();
        // O(log N) insertion
        list.insert(new_item);
        return true;
      }
    }
    let sm = ScoreMember { score, member };
    self.map.insert(key.clone(), sm.clone());
    // O(log N) insertion
    list.insert(sm);
    false
  }

  /// Removes a member.
  /// # Time Complexity 时间复杂度
  /// - O(log N)
  fn rm(&self, member: impl Borrow<Self::K>) -> bool {
    if let Some((_, sm)) = self.map.remove(member.borrow()) {
      let mut list = self.list.write();
      // O(log N) removal
      list.remove(&sm).is_some()
    } else {
      false
    }
  }

  /// Removes a range of members in the sorted set, with scores ordered from low to high.
  /// The `range` is a 0-based.
  /// Returns the number of members removed.
  /// 按排名（0-based）从低到高，删除排序集合中指定范围的成员。
  /// 返回被删除的成员数量。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(K * log N) where K is the size of the range.
  /// - O(K * log N)，其中 K 是范围的大小。
  fn rm_range_by_rank(&self, range: impl RangeBounds<usize>) -> usize {
    let mut list = self.list.write();
    let len = list.len();
    let range = resolve_range(range, len);

    if range.is_empty() {
      return 0;
    }

    let mut removed_count = 0;
    for i in (range.start..range.end).rev() {
      let item = list.remove_index(i);
      self.map.remove(item.member.borrow());
      removed_count += 1;
    }
    removed_count
  }

  /// Gets the score of a member.
  /// 获取成员的分数。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(1) on average.
  /// - 平均时间复杂度为 O(1)。
  fn score(&self, member: impl Borrow<Self::K>) -> Option<S> {
    self.map.get(member.borrow()).map(|r| r.score.clone())
  }

  /// Gets the 0-based rank of a member, ordered from low to high score.
  /// 获取成员的排名（0-based），按分数从低到高排序。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(log N)
  fn rank(&self, member: impl Borrow<Self::K>) -> Option<usize> {
    self
      .map
      .get(member.borrow())
      .and_then(|r| self.list.read().index_of(r.value()))
  }

  /// Returns the number of members in the zset.
  /// 返回 zset 中的成员数量。
  fn len(&self) -> usize {
    self.map.len()
  }

  /// Returns `true` if the zset contains no members.
  /// 如果 zset 为空，则返回 `true`。
  fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  /// Checks if a member exists in the zset.
  /// 检查成员是否存在于 zset 中。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(1) on average.
  /// - 平均时间复杂度为 O(1)。
  fn contains(&self, member: impl Borrow<Self::K>) -> bool {
    self.map.contains_key(member.borrow())
  }

  /// Returns a range of members, ordered by score from low to high.
  /// 返回一个成员范围，按分数从低到高排序。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(log N + K) where N is the number of elements and K is the size of the range.
  /// - O(log N + K)，其中 N 是元素数量，K 是范围的大小。
  fn range(&self, range: impl RangeBounds<usize>) -> Vec<Arc<M>> {
    with_range_iter!(self, range, |iter: skiplist::ordered_skiplist::Iter<
      ScoreMember<K, M, S>,
    >| {
      iter.map(|item| item.member.inner.clone()).collect()
    })
  }

  /// Returns a range of members with their scores, ordered by score from low to high.
  /// 返回一个带分数的成员范围，按分数从低到高排序。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(log N + K) where N is the number of elements and K is the size of the range.
  /// - O(log N + K)，其中 N 是元素数量，K 是范围的大小。
  fn range_with_scores(&self, range: impl RangeBounds<usize>) -> Vec<(Arc<M>, S)> {
    with_range_iter!(self, range, |iter: skiplist::ordered_skiplist::Iter<
      ScoreMember<K, M, S>,
    >| {
      iter
        .map(|item| (item.member.inner.clone(), item.score.clone()))
        .collect()
    })
  }

  /// Gets the member at the given 0-based rank.
  /// 获取指定排名（0-based）的成员。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(log N) for skiplist lookup.
  /// - skiplist 查找的时间复杂度为 O(log N)。
  fn get(&self, rank: usize) -> Option<Arc<M>> {
    self.with_item_by_rank(rank, |item| item.member.inner.clone())
  }

  /// Gets the member and score at the given 0-based rank.
  /// 获取指定排名（0-based）的成员和分数。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(log N) for skiplist lookup.
  /// - skiplist 查找的时间复杂度为 O(log N)。
  fn get_with_score(&self, rank: usize) -> Option<(Arc<M>, S)> {
    self.with_item_by_rank(rank, |item| (item.member.inner.clone(), item.score.clone()))
  }
}

use std::ops::{Add, BitAnd, BitAndAssign, BitOr, BitOrAssign};

// Union operator: zset1 | zset2
impl<'b, K, M, S> BitOr<&'b Zset<K, M, S>> for &Zset<K, M, S>
where
  K: Key,
  M: Member<K>,
  S: Score + Add<Output = S>,
{
  type Output = Zset<K, M, S>;

  /// Performs a union of two Zsets.
  /// The scores of common members are summed.
  /// 对两个 Zset 执行并集操作。
  /// 共同成员的分数会被相加。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(L * log L) where L is the size of the larger set.
  /// - O(L * log L)，其中 L 是较大集合的大小。
  fn bitor(self, rhs: &'b Zset<K, M, S>) -> Self::Output {
    let (smaller, larger) = if self.len() < rhs.len() {
      (self, rhs)
    } else {
      (rhs, self)
    };

    let mut result = Zset::new();
    for (member, score) in larger.range_with_scores(..) {
      result.add(member, score);
    }

    result |= smaller;
    result
  }
}

// Union-assign operator: zset1 |= zset2
#[allow(clippy::suspicious_op_assign_impl)]
impl<'a, K, M, S> BitOrAssign<&'a Zset<K, M, S>> for Zset<K, M, S>
where
  K: Key,
  M: Member<K>,
  S: Score + Add<Output = S>,
{
  /// Performs a union operation and assigns the result to the left-hand side.
  /// The scores of common members are summed.
  /// 执行并集操作并将结果赋给左侧。
  /// 共同成员的分数会被相加。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(M * log N) where M is the size of `rhs` and N is the size of `self`.
  /// - O(M * log N)，其中 M 是 `rhs` 的大小，N 是 `self` 的大小。
  fn bitor_assign(&mut self, rhs: &'a Zset<K, M, S>) {
    for (member, score) in rhs.range_with_scores(0..rhs.len()) {
      let new_score = self
        .score(member.as_ref().borrow())
        .map_or_else(|| score.clone(), |s| s + score.clone());
      self.add(member, new_score);
    }
  }
}

// Intersection operator: zset1 & zset2
impl<'b, K, M, S> BitAnd<&'b Zset<K, M, S>> for &Zset<K, M, S>
where
  K: Key,
  M: Member<K>,
  S: Score + Add<Output = S>,
{
  type Output = Zset<K, M, S>;

  /// Performs an intersection of two Zsets.
  /// The scores of common members are summed.
  /// 对两个 Zset 执行交集操作。
  /// 共同成员的分数会被相加。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(S * log S) where S is the size of the smaller set.
  /// - O(S * log S)，其中 S 是较小集合的大小。
  fn bitand(self, rhs: &'b Zset<K, M, S>) -> Self::Output {
    let new_zset = Zset::new();
    let (smaller, larger) = if self.len() <= rhs.len() {
      (self, rhs)
    } else {
      (rhs, self)
    };

    for (member, score1) in smaller.range_with_scores(0..smaller.len()) {
      if let Some(score2) = larger.score(member.as_ref().borrow()) {
        new_zset.add(member, score1 + score2);
      }
    }
    new_zset
  }
}

// Intersection-assign operator: zset1 &= zset2
#[allow(clippy::suspicious_op_assign_impl)]
impl<'a, K, M, S> BitAndAssign<&'a Zset<K, M, S>> for Zset<K, M, S>
where
  K: Key,
  M: Member<K>,
  S: Score + Add<Output = S>,
{
  /// Performs an intersection operation and assigns the result to the left-hand side.
  /// Members not present in the right-hand side are rmd.
  /// The scores of remaining common members are summed.
  /// 执行交集操作并将结果赋给左侧。
  /// 不在右侧的成员会被移除。
  /// 剩余共同成员的分数会被相加。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(N * log N) where N is the size of `self`.
  /// - O(N * log N)，其中 N 是 `self` 的大小。
  fn bitand_assign(&mut self, rhs: &'a Zset<K, M, S>) {
    let to_remove: Vec<K> = self
      .map
      .iter()
      .filter(|item| rhs.score(item.key()).is_none())
      .map(|item| item.key().clone())
      .collect();

    for k in to_remove {
      self.rm(&k);
    }

    for mut item in self.map.iter_mut() {
      if let Some(score2) = rhs.score(item.key()) {
        let mut list = self.list.write();
        list.remove(&*item);
        item.score = item.score.clone() + score2;
        list.insert(item.clone());
      }
    }
  }
}
