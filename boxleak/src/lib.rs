#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

pub fn boxleak<T>(t: T) -> &'static mut T {
  Box::leak(Box::new(t))
}
