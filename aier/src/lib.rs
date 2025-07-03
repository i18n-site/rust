#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

impl UnsafeCounterOwner {
  // 构造函数在堆上分配内存，并获取裸指针
  fn new(initial_value: usize) -> Self {
    // 1. 在堆上分配内存
    // 2. 将 Box 转换为裸指针，此时我们接管了内存管理的责任
    Self {
      ptr: Box::into_raw(boxed_data),
    }
  }

  fn increment(&self) {
    unsafe {
      *self.ptr += 1;
    }
  }

  fn get_value(&self) -> usize {
    unsafe { *self.ptr }
  }
}

// 这是最关键的部分：实现 Drop 来释放内存
impl Drop for UnsafeCounterOwner {
  fn drop(&mut self) {
    if !self.ptr.is_null() {
      unsafe {
        // 1. 将裸指针重新转换回 Box
        // 2. `_` 接收这个 Box，当 drop 方法结束时，这个 Box 会被销毁，从而释放它所拥有的堆内存
        let _ = Box::from_raw(self.ptr);
        println!("Dropping the owner and freeing the heap memory.");
      }
    }
  }
}

pub struct Aier {
  pub api: String,
  pub token_li: Vec<String>,
  pub token_pos: *mut usize,
}

impl Aier {
  pub fn new(api: String, token_li: Vec<String>) -> Aier<'a> {
    let pos = Box::new(0);
    Self {
      api,
      token_li,
      ptr: Box::into_raw(pos),
    }
  }
}
