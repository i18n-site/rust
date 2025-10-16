use rand::Rng;

pub fn iter<T>(li: &[T]) -> impl Iterator<Item = &T> {
  let offset = rand::rng().random_range(0..li.len());
  li[offset..].iter().chain(li[..offset].iter())
}
