use std::{cmp::Reverse, collections::BinaryHeap};

use gxhash::HashMap;

pub fn topk(n: usize, map: HashMap<String, u64>) -> impl Iterator<Item = (String, u64)> {
  let mut heap = BinaryHeap::with_capacity(n);

  for (key, value) in map {
    if heap.len() < n {
      heap.push((Reverse(value), key));
    } else if let Some(&(Reverse(top_value), _)) = heap.peek() {
      if value > top_value {
        heap.pop();
        heap.push((Reverse(value), key));
      }
    }
  }

  heap
    .into_iter()
    .map(|(Reverse(ts), rel)| (rel, ts))
    .rev()
}
