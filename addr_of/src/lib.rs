pub fn addr_of<T>(ptr: &T) -> usize {
  ptr as *const T as usize
}
