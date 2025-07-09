# drop

## test
```rust
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
```

### test out
```
    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running unittests src/lib.rs (/tmp/rust/target/debug/deps/drop-312dac5912c289bd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/main.rs (/tmp/rust/target/debug/deps/main-35b2cf4e21425308)

running 1 test
  INFO main: drop/tests/main.rs:14: > obj Test(1)
[drop/src/lib.rs:51:5] "drop" = "drop"
  INFO main: drop/tests/main.rs:14: > obj Test(1)
[drop/src/lib.rs:51:5] "drop" = "drop"
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests drop

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## code

```rust
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
```

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
