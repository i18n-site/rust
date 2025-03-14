use std::marker::PhantomData;

pub use boxleak::boxleak;

pub struct Leak<T> {
  ptr: usize,
  _marker: PhantomData<T>,
}

pub struct Wrap<T: 'static> {
  _leak: Leak<T>,
  pub ptr: &'static mut T,
}

pub fn leak<T>(object: T) -> Wrap<T> {
  let ptr = boxleak(object);
  Wrap {
    _leak: Leak::<T> {
      ptr: ptr as *mut T as usize,
      _marker: PhantomData,
    },
    ptr,
  }
}

impl<T> Drop for Leak<T> {
  fn drop(&mut self) {
    unsafe {
      drop(Box::from_raw(self.ptr as *mut T));
    }
  }
}
