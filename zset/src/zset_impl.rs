use std::{hash::Hash, ops::Range, sync::Arc};

use dashmap::DashMap;
use parking_lot::{RwLock, RwLockReadGuard};
use sorted_vec::SortedVec;

use crate::{Api, score_member::ScoreMember};

/// A thread-safe, Redis-like sorted set implementation.
/// 一个线程安全的、类似 Redis 的排序集合实现。
#[derive(Debug)]
pub struct Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static,
  S: Ord + Send + Sync + 'static + Clone,
{
  map: DashMap<Arc<M>, S>,
  vec: RwLock<SortedVec<ScoreMember<M, S>>>,
}

impl<M, S> Clone for Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static + Clone,
  S: Ord + Send + Sync + 'static + Clone,
{
  /// Creates a new Zset with the same elements as the original.
  /// This is an expensive operation as it clones all elements.
  /// 创建一个与原始 Zset 具有相同元素的新 Zset。
  /// 这是一个昂贵的操作，因为它会克隆所有元素。
  fn clone(&self) -> Self {
    let vec_reader = self.vec.read();
    let new_vec = vec_reader.clone();
    let new_map = DashMap::with_capacity(new_vec.len());
    for item in new_vec.iter() {
      new_map.insert(item.member.clone(), item.score.clone());
    }
    Self {
      map: new_map,
      vec: RwLock::new(new_vec),
    }
  }
}

impl<M, S> Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static,
  S: Ord + Send + Sync + 'static + Clone,
{
  /// Creates a new, empty Zset.
  /// 创建一个新的空 Zset。
  pub fn new() -> Self {
    Self {
      map: DashMap::new(),
      vec: RwLock::new(SortedVec::new()),
    }
  }

  /// Returns a slice of the underlying sorted vector within the specified range.
  /// 返回指定范围内底层排序向量的切片。
  fn slice_range(
    &self,
    range: Range<usize>,
  ) -> (
    RwLockReadGuard<'_, SortedVec<ScoreMember<M, S>>>,
    Range<usize>,
  ) {
    let r = self.vec.read();
    let start = range.start.min(r.len());
    let end = range.end.min(r.len());
    (r, start..end)
  }
}

impl<M, S> Default for Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static,
  S: Ord + Send + Sync + 'static + Clone,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<M, S> Api<M, S> for Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static,
  S: Ord + Send + Sync + 'static + Clone,
{
  /// Adds a member with a score.
  /// If the member already exists, its score is updated.
  /// 添加一个成员及其分数。
  /// 如果成员已存在，则更新其分数。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(N) where N is the number of elements. This is because after finding the position in O(log N), it may take O(N) to shift elements for insertion.
  /// - O(N)，其中 N 是元素的数量。这是因为在 O(log N) 时间内找到位置后，可能需要 O(N) 的时间来移动元素以进行插入。
  fn add(&self, member: M, score: S) -> bool {
    let member_arc = Arc::new(member);
    match self.map.entry(member_arc) {
      dashmap::mapref::entry::Entry::Occupied(mut o) => {
        let old_score = o.get().clone();
        if old_score == score {
          return false; // Score is the same, no update needed
        }
        // Update score and vec
        let member_arc = o.key().clone();
        o.insert(score.clone());
        let mut vec = self.vec.write();
        vec.remove_item(&ScoreMember {
          score: old_score,
          member: member_arc.clone(),
        });
        vec.insert(ScoreMember {
          score,
          member: member_arc,
        });
        true
      }
      dashmap::mapref::entry::Entry::Vacant(v) => {
        let member_arc = v.key().clone();
        v.insert(score.clone());
        self.vec.write().insert(ScoreMember {
          score,
          member: member_arc,
        });
        false
      }
    }
  }

  /// Removes a member.
  /// # Time Complexity 时间复杂度
  /// - O(N) where N is the number of elements. This is because after finding the element, it may take O(N) to shift elements to fill the gap.
  /// - O(N)，其中 N 是集合中的元素数量。这是因为找到元素后，可能需要 O(N) 的时间来移动元素以填补空缺。
  fn remove(&self, member: &M) -> bool {
    if let Some((member, score)) = self.map.remove(member) {
      self.vec.write().remove_item(&ScoreMember { score, member });
      true
    } else {
      false
    }
  }

  /// Gets the score of a member.
  /// 获取成员的分数。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(1) on average.
  /// - 平均时间复杂度为 O(1)。
  fn score(&self, member: &M) -> Option<S> {
    self.map.get(member).map(|r| r.value().clone())
  }

  /// Gets the 0-based rank of a member, ordered from low to high score.
  /// 获取成员的排名（0-based），按分数从低到高排序。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(log N) where N is the number of elements in the set.
  /// - O(log N)，其中 N 是集合中的元素数量。
  fn rank(&self, member: &M) -> Option<usize> {
    self.map.get(member).and_then(|r| {
      self
        .vec
        .read()
        .binary_search(&ScoreMember {
          score: r.value().clone(),
          member: r.key().clone(),
        })
        .ok()
    })
  }

  fn card(&self) -> usize {
    self.map.len()
  }

  fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  /// Returns a range of members, ordered by score from low to high.
  /// 返回一个成员范围，按分数从低到高排序。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(K) where K is the size of the range.
  /// - O(K)，其中 K 是范围的大小。
  fn range(&self, range: Range<usize>) -> Vec<Arc<M>> {
    let (r, range) = self.slice_range(range);
    r[range].iter().map(|item| item.member.clone()).collect()
  }

  /// Returns a range of members with their scores, ordered by score from low to high.
  /// 返回一个带分数的成员范围，按分数从低到高排序。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(K) where K is the size of the range.
  /// - O(K)，其中 K 是范围的大小。
  fn range_with_scores(&self, range: Range<usize>) -> Vec<(Arc<M>, S)> {
    let (r, range) = self.slice_range(range);
    r[range]
      .iter()
      .map(|item| (item.member.clone(), item.score.clone()))
      .collect()
  }

  /// Gets the member at the given 0-based rank.
  /// 获取指定排名（0-based）的成员。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(1) for the vector lookup.
  /// - 向量查找的时间复杂度为 O(1)。
  fn get(&self, rank: usize) -> Option<Arc<M>> {
    self.vec.read().get(rank).map(|item| item.member.clone())
  }

  /// Gets the member and score at the given 0-based rank.
  /// 获取指定排名（0-based）的成员和分数。
  ///
  /// # Time Complexity 时间复杂度
  /// - O(1) for the vector lookup.
  /// - 向量查找的时间复杂度为 O(1)。
  fn get_with_score(&self, rank: usize) -> Option<(Arc<M>, S)> {
    self
      .vec
      .read()
      .get(rank)
      .map(|item| (item.member.clone(), item.score.clone()))
  }
}

use std::ops::{Add, BitAnd, BitAndAssign, BitOr, BitOrAssign};

// Union operator: zset1 | zset2
impl<'b, M, S> BitOr<&'b Zset<M, S>> for &Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static + Clone,
  S: Ord + Send + Sync + 'static + Clone + Add<Output = S>,
{
  type Output = Zset<M, S>;

  /// Performs a union of two Zsets.
  /// The scores of common members are summed.
  /// 对两个 Zset 执行并集操作。
  /// 共同成员的分数会被相加。
  fn bitor(self, rhs: &'b Zset<M, S>) -> Self::Output {
    let mut new_zset = self.clone();
    new_zset |= rhs;
    new_zset
  }
}

// Union-assign operator: zset1 |= zset2
#[allow(clippy::suspicious_op_assign_impl)]
impl<'a, M, S> BitOrAssign<&'a Zset<M, S>> for Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static + Clone,
  S: Ord + Send + Sync + 'static + Clone + Add<Output = S>,
{
  /// Performs a union operation and assigns the result to the left-hand side.
  /// The scores of common members are summed.
  /// 执行并集操作并将结果赋给左侧。
  /// 共同成员的分数会被相加。
  fn bitor_assign(&mut self, rhs: &'a Zset<M, S>) {
    for (member, score) in rhs.range_with_scores(0..rhs.card()) {
      let new_score = self
        .score(&member)
        .map_or_else(|| score.clone(), |s| s + score.clone());
      self.add((*member).clone(), new_score);
    }
  }
}

// Intersection operator: zset1 & zset2
impl<'b, M, S> BitAnd<&'b Zset<M, S>> for &Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static + Clone,
  S: Ord + Send + Sync + 'static + Clone + Add<Output = S>,
{
  type Output = Zset<M, S>;

  /// Performs an intersection of two Zsets.
  /// The scores of common members are summed.
  /// 对两个 Zset 执行交集操作。
  /// 共同成员的分数会被相加。
  fn bitand(self, rhs: &'b Zset<M, S>) -> Self::Output {
    let new_zset = Zset::new();
    let (smaller, larger) = if self.card() <= rhs.card() {
      (self, rhs)
    } else {
      (rhs, self)
    };

    for (member, score1) in smaller.range_with_scores(0..smaller.card()) {
      if let Some(score2) = larger.score(&member) {
        new_zset.add((*member).clone(), score1 + score2);
      }
    }
    new_zset
  }
}

// Intersection-assign operator: zset1 &= zset2
#[allow(clippy::suspicious_op_assign_impl)]
impl<'a, M, S> BitAndAssign<&'a Zset<M, S>> for Zset<M, S>
where
  M: Eq + Hash + Ord + Send + Sync + 'static + Clone,
  S: Ord + Send + Sync + 'static + Clone + Add<Output = S>,
{
  /// Performs an intersection operation and assigns the result to the left-hand side.
  /// Members not present in the right-hand side are removed.
  /// The scores of remaining common members are summed.
  /// 执行交集操作并将结果赋给左侧。
  /// 不在右侧的成员会被移除。
  /// 剩余共同成员的分数会被相加。
  fn bitand_assign(&mut self, rhs: &'a Zset<M, S>) {
    let members_to_process: Vec<_> = self
      .range_with_scores(0..self.card())
      .into_iter()
      .map(|(member, score)| ((*member).clone(), score))
      .collect();

    for (member, score1) in members_to_process {
      if let Some(score2) = rhs.score(&member) {
        self.add(member, score1 + score2);
      } else {
        self.remove(&member);
      }
    }
  }
}
