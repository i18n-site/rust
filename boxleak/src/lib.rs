pub fn boxleak<T>(t: T) -> &'static mut T {
  Box::leak(Box::new(t))
}
