use std::{
  sync::{Arc, Barrier},
  thread,
};

use ordered_float::OrderedFloat;
use zset::{Api, Zset};

#[test]
fn test_add_and_len() {
  // Test adding new elements and checking leninality.
  // 测试添加新元素和检查基数。
  let zset = Zset::<&str, &str, i32>::new();
  assert!(!zset.add("one", 1));
  assert_eq!(zset.len(), 1);
  assert!(!zset.add("two", 2));
  assert_eq!(zset.len(), 2);
  // Test updating an existing element.
  // 测试更新现有元素。
  assert!(zset.add("one", 10));
  assert_eq!(zset.len(), 2);
  // After update, "two" has score 2 (rank 0), "one" has score 10 (rank 1)
  // 更新后，"two" 的分数是 2 (排名 0)，"one" 的分数是 10 (排名 1)
  assert_eq!(zset.get(0).map(|v| *v), Some("two"));
  assert_eq!(
    zset.get_with_score(0).map(|(m, s)| (*m, s)),
    Some(("two", 2))
  );
  assert_eq!(zset.get(1).map(|v| *v), Some("one"));
  assert_eq!(
    zset.get_with_score(1).map(|(m, s)| (*m, s)),
    Some(("one", 10))
  );
}

#[test]
fn test_rm() {
  // Test removing elements.
  // 测试移除元素。
  let zset: Zset<&str, &str, i32> = Zset::new();
  zset.add("one", 1);
  zset.add("two", 2);
  assert!(zset.rm(&"one"));
  assert_eq!(zset.len(), 1);
  assert_eq!(zset.score(&"one"), None);
  // Test removing a non-existent element.
  // 测试移除不存在的元素。
  assert!(!zset.rm(&"three"));
  assert_eq!(zset.len(), 1);
}

#[test]
fn test_score_and_rank() {
  // Test getting score and rank.
  // 测试获取分数和排名。
  let zset: Zset<&str, &str, i32> = Zset::new();
  zset.add("one", 10);
  zset.add("two", 20);
  zset.add("three", 30);

  assert_eq!(zset.score(&"two"), Some(20));
  assert_eq!(zset.score(&"non-existent"), None);

  // Ranks are 0-based.
  // 排名是 0-based 的。
  assert_eq!(zset.rank(&"one"), Some(0));
  assert_eq!(zset.rank(&"two"), Some(1));
  assert_eq!(zset.rank(&"three"), Some(2));
  assert_eq!(zset.rank(&"non-existent"), None);

  // Test rank after score update.
  // 测试分数更新后的排名。
  zset.add("one", 40);
  assert_eq!(zset.rank(&"two"), Some(0));
  assert_eq!(zset.rank(&"three"), Some(1));
  assert_eq!(zset.rank(&"one"), Some(2));
}

#[test]
fn test_range() {
  // Test getting ranges of elements.
  // 测试获取元素范围。
  let zset: Zset<&str, &str, i32> = Zset::new();
  zset.add("c", 3);
  zset.add("d", 4);
  zset.add("e", 5);
  zset.add("a", 1);
  zset.add("b", 2);

  // Test various ranges.
  // 测试不同范围。
  let r: Vec<&str> = zset.range(0..3).iter().map(|v| **v).collect();
  assert_eq!(r, vec!["a", "b", "c"]);
  let r: Vec<&str> = zset.range(2..5).iter().map(|v| **v).collect();
  assert_eq!(r, vec!["c", "d", "e"]);
  let r: Vec<&str> = zset.range(0..zset.len()).iter().map(|v| **v).collect();
  assert_eq!(r, vec!["a", "b", "c", "d", "e"]);

  // Test RangeBounds
  let r: Vec<&str> = zset.range(..3).iter().map(|v| **v).collect();
  assert_eq!(r, vec!["a", "b", "c"]);
  let r: Vec<&str> = zset.range(2..).iter().map(|v| **v).collect();
  assert_eq!(r, vec!["c", "d", "e"]);
  let r: Vec<&str> = zset.range(..).iter().map(|v| **v).collect();
  assert_eq!(r, vec!["a", "b", "c", "d", "e"]);
  let r: Vec<&str> = zset.range(1..=3).iter().map(|v| **v).collect();
  assert_eq!(r, vec!["b", "c", "d"]);

  // Test out of bounds ranges.
  // 测试超出边界的范围。
  assert!(zset.range(5..10).is_empty());
  assert!(zset.range(10..12).is_empty());
  assert!(zset.range(3..3).is_empty());
}

#[test]
fn test_range_with_scores() {
  // Test getting ranges with scores.
  // 测试获取带分数的范围。
  let zset: Zset<&str, &str, i32> = Zset::new();
  zset.add("a", 1);
  zset.add("b", 2);
  zset.add("c", 3);

  let expected = vec![("a", 1), ("b", 2), ("c", 3)];
  let r: Vec<(&str, i32)> = zset
    .range_with_scores(0..3)
    .iter()
    .map(|(m, s)| (**m, *s))
    .collect();
  assert_eq!(r, expected);
  let r: Vec<(&str, i32)> = zset
    .range_with_scores(0..zset.len())
    .iter()
    .map(|(m, s)| (**m, *s))
    .collect();
  assert_eq!(r, expected);

  let expected_subset = vec![("b", 2), ("c", 3)];
  let r: Vec<(&str, i32)> = zset
    .range_with_scores(1..3)
    .iter()
    .map(|(m, s)| (**m, *s))
    .collect();
  assert_eq!(r, expected_subset);
}

#[test]
fn test_rm_range_by_rank() {
  // Test removing elements by rank range.
  // 按排名范围测试删除元素。
  let zset: Zset<&str, &str, i32> = Zset::new();
  zset.add("a", 1);
  zset.add("b", 2);
  zset.add("c", 3);
  zset.add("d", 4);
  zset.add("e", 5);

  // Remove "c" and "d" (ranks 2 and 3)
  // 删除 "c" 和 "d" (排名 2 和 3)
  assert_eq!(zset.rm_range_by_rank(2..4), 2);
  assert_eq!(zset.len(), 3);
  assert_eq!(zset.score(&"c"), None);
  assert_eq!(zset.score(&"d"), None);
  assert_eq!(zset.rank(&"a"), Some(0));
  assert_eq!(zset.rank(&"b"), Some(1));
  assert_eq!(zset.rank(&"e"), Some(2));

  // Reset zset
  zset.rm_range_by_rank(..);
  assert_eq!(zset.len(), 0);
  zset.add("a", 1);
  zset.add("b", 2);
  zset.add("c", 3);
  zset.add("d", 4);
  zset.add("e", 5);

  // Test with other RangeBounds
  // 使用其他 RangeBounds 进行测试
  // Remove "a", "b" (ranks 0, 1)
  assert_eq!(zset.rm_range_by_rank(..2), 2);
  assert_eq!(zset.len(), 3);
  assert_eq!(zset.score(&"a"), None);
  assert_eq!(zset.score(&"b"), None);

  // Remove "e" (rank 2, originally 4)
  assert_eq!(zset.rm_range_by_rank(2..), 1);
  assert_eq!(zset.len(), 2);
  assert_eq!(zset.score(&"e"), None);

  // Remove "c", "d"
  assert_eq!(zset.rm_range_by_rank(..), 2);
  assert_eq!(zset.len(), 0);

  // Test inclusive range
  zset.add("a", 1);
  zset.add("b", 2);
  zset.add("c", 3);
  assert_eq!(zset.rm_range_by_rank(0..=1), 2);
  assert_eq!(zset.len(), 1);
  assert_eq!(zset.score(&"c"), Some(3));
}

#[test]
fn test_concurrent_add() {
  // Test concurrent additions from multiple threads.
  // 测试多线程并发添加。
  let zset: Arc<Zset<String, String, i32>> = Arc::new(Zset::new());
  let barrier = Arc::new(Barrier::new(10));
  let mut handles = vec![];

  for i in 0..10 {
    let zset_clone = zset.clone();
    let barrier_clone = barrier.clone();
    handles.push(thread::spawn(move || {
      let member = format!("member-{}", i);
      let score = i as i32;
      barrier_clone.wait();
      // Move the String into the zset, making M = String
      // 将 String 移入 zset，使 M = String
      zset_clone.add(member, score);
    }));
  }

  for handle in handles {
    handle.join().unwrap();
  }

  assert_eq!(zset.len(), 10);
  for i in 0..10 {
    let member = format!("member-{}", i);
    // Pass a &String to score and rank
    // 向 score 和 rank 传递 &String
    assert_eq!(zset.score(&member), Some(i as i32));
    assert_eq!(zset.rank(&member), Some(i));
  }
}

#[test]
fn test_concurrent_add_rm() {
  // Test concurrent additions and removals.
  // 测试并发添加和删除。
  let zset: Arc<Zset<String, String, i64>> = Arc::new(Zset::new());
  let barrier = Arc::new(Barrier::new(100));

  // Add 100 elements concurrently.
  // 并发添加 100 个元素。
  let mut handles = vec![];
  for i in 0..100 {
    let zset_clone = zset.clone();
    let barrier_clone = barrier.clone();
    handles.push(thread::spawn(move || {
      barrier_clone.wait();
      zset_clone.add(i.to_string(), i as i64);
    }));
  }
  for handle in handles {
    handle.join().unwrap();
  }
  assert_eq!(zset.len(), 100);

  // Concurrently rm even-numbered elements.
  // 并发删除偶数元素。
  let barrier = Arc::new(Barrier::new(50));
  let mut handles = vec![];
  for i in (0..100).filter(|&x| x % 2 == 0) {
    let zset_clone = zset.clone();
    let barrier_clone = barrier.clone();
    handles.push(thread::spawn(move || {
      barrier_clone.wait();
      zset_clone.rm(&i.to_string());
    }));
  }
  for handle in handles {
    handle.join().unwrap();
  }

  // Only odd-numbered elements should remain.
  // 应该只剩下奇数元素。
  assert_eq!(zset.len(), 50);
  assert_eq!(zset.score(&"49".to_string()), Some(49));
  assert_eq!(zset.score(&"50".to_string()), None);
}

#[test]
fn test_float_scores() {
  // Test Zset with floating-point scores.
  // 测试使用浮点数分数的 Zset。
  let zset = Zset::<&str, &str, OrderedFloat<f64>>::new();
  zset.add("one", OrderedFloat(1.5));
  zset.add("two", OrderedFloat(2.5));
  zset.add("neg_one", OrderedFloat(-1.0));
  zset.add("zero", OrderedFloat(0.0));

  assert_eq!(zset.len(), 4);

  // Check scores.
  // 检查分数。
  assert_eq!(zset.score(&"two"), Some(OrderedFloat(2.5)));

  // Check ranks (low to high).
  // 检查排名（从低到高）。
  assert_eq!(zset.rank(&"neg_one"), Some(0));
  assert_eq!(zset.rank(&"zero"), Some(1));
  assert_eq!(zset.rank(&"one"), Some(2));
  assert_eq!(zset.rank(&"two"), Some(3));

  let expected_range = vec!["neg_one", "zero", "one", "two"];
  let r: Vec<&str> = zset.range(0..zset.len()).iter().map(|v| **v).collect();
  assert_eq!(r, expected_range);

  // Check range with scores.
  // 检查带分数的范围。
  let expected_range_with_scores = vec![
    ("neg_one", OrderedFloat(-1.0)),
    ("zero", OrderedFloat(0.0)),
    ("one", OrderedFloat(1.5)),
    ("two", OrderedFloat(2.5)),
  ];
  let r: Vec<(&str, OrderedFloat<f64>)> = zset
    .range_with_scores(0..zset.len())
    .iter()
    .map(|(m, s)| (**m, *s))
    .collect();
  assert_eq!(r, expected_range_with_scores);
}

#[test]
fn test_operators() {
  // Test union
  // 测试并集
  let zset1: Zset<String, String, i32> = Zset::new();
  zset1.add("a".to_string(), 1);
  zset1.add("b".to_string(), 2);

  let zset2: Zset<String, String, i32> = Zset::new();
  zset2.add("b".to_string(), 3);
  zset2.add("c".to_string(), 4);

  let zset_union = &zset1 | &zset2;
  assert_eq!(zset_union.len(), 3);
  assert_eq!(zset_union.score(&"a".to_string()), Some(1));
  assert_eq!(zset_union.score(&"b".to_string()), Some(5)); // 2 + 3
  assert_eq!(zset_union.score(&"c".to_string()), Some(4));

  // Test union-assign
  // 测试并集赋值
  let mut zset1_for_assign: Zset<String, String, i32> = Zset::new();
  zset1_for_assign.add("a".to_string(), 1);
  zset1_for_assign.add("b".to_string(), 2);
  zset1_for_assign |= &zset2;
  assert_eq!(zset1_for_assign.len(), 3);
  assert_eq!(zset1_for_assign.score(&"a".to_string()), Some(1));
  assert_eq!(zset1_for_assign.score(&"b".to_string()), Some(5));
  assert_eq!(zset1_for_assign.score(&"c".to_string()), Some(4));

  // Test intersection
  // 测试交集
  let zset_intersection = &zset1 & &zset2;
  assert_eq!(zset_intersection.len(), 1);
  assert_eq!(zset_intersection.score(&"b".to_string()), Some(5)); // 2 + 3

  // Test intersection-assign
  // 测试交集赋值
  let mut zset1_for_assign2: Zset<String, String, i32> = Zset::new();
  zset1_for_assign2.add("a".to_string(), 1);
  zset1_for_assign2.add("b".to_string(), 2);
  zset1_for_assign2 &= &zset2;
  assert_eq!(zset1_for_assign2.len(), 1);
  assert_eq!(zset1_for_assign2.score(&"b".to_string()), Some(5));
  assert_eq!(zset1_for_assign2.score(&"a".to_string()), None);
}

#[test]
fn test_is_empty_and_len() {
  let zset = Zset::<&str, &str, i32>::new();
  assert!(zset.is_empty());
  assert_eq!(zset.len(), 0);

  zset.add("one", 1);
  assert!(!zset.is_empty());
  assert_eq!(zset.len(), 1);

  zset.rm(&"one");
  assert!(zset.is_empty());
  assert_eq!(zset.len(), 0);
}

#[test]
#[cfg(feature = "card")]
fn test_card() {
  // Test the card alias for len.
  // 测试 len 的别名 card。
  let zset = Zset::<&str, &str, i32>::new();
  zset.add("one", 1);
  zset.add("two", 2);
  assert_eq!(zset.card(), 2);
  assert_eq!(zset.card(), zset.len());
  zset.rm(&"one");
  assert_eq!(zset.card(), 1);
}

#[test]
#[ignore] // This test is slow and is meant for stress testing.
fn test_concurrent_stress() {
  use rand::Rng;

  let zset: Arc<Zset<String, String, i32>> = Arc::new(Zset::new());
  let num_threads = 50;
  let num_ops_per_thread = 1000;
  let member_space = 1000;

  let mut handles = vec![];

  for _ in 0..num_threads {
    let zset_clone = zset.clone();
    handles.push(thread::spawn(move || {
      let mut rng = rand::rng();
      for _ in 0..num_ops_per_thread {
        let op = rng.random_range(0..5);
        let member = rng.random_range(0..member_space).to_string();

        match op {
          0 => {
            let score = rng.random_range(0..10000);
            zset_clone.add(member, score);
          }
          1 => {
            zset_clone.rm(&member);
          }
          2 => {
            zset_clone.rank(&member);
          }
          3 => {
            zset_clone.score(&member);
          }
          4 => {
            let len = zset_clone.len();
            if len > 0 {
              let start = rng.random_range(0..len);
              let end = rng.random_range(start..=len);
              zset_clone.range(start..end);
            }
          }
          _ => unreachable!(),
        }
      }
    }));
  }
  for handle in handles {
    handle.join().unwrap();
  }
}
