use std::marker::PhantomData;

pub use boxleak::boxleak;
pub use paste::paste;

pub struct Leak<T> {
  ptr: usize,
  _marker: PhantomData<T>,
}

pub struct Wrap<T: 'static> {
  _leak: Leak<T>,
  pub ptr: &'static mut T,
}

#[macro_export]
macro_rules! help {
  ($name:ident $new:expr) => {
    $new
  };
  ($name:ident) => {
    $name
  };
}

#[macro_export]
macro_rules! leak {
  ($($name:ident $(= $new: expr)?),+) => {
    $(
    $crate::paste! {
      let [<__drop_ $name>] = $crate::_leak($crate::help!($name $($new)?));
      let $name = [<__drop_ $name>].ptr;
    }
    )+
  };
}

pub fn _leak<T>(object: T) -> Wrap<T> {
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
    // dbg!("drop");
    unsafe {
      drop(Box::from_raw(self.ptr as *mut T));
    }
  }
}
