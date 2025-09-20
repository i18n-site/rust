use indexmap::IndexMap;

pub fn sortmap<K: std::cmp::Ord + std::hash::Hash, V>(mut li: Vec<(K, V)>) -> IndexMap<K, V> {
  let mut t = IndexMap::new();
  li.sort_by(|a, b| a.0.cmp(&b.0));
  for (key, value) in li {
    t.insert(key, value);
  }
  t
}
