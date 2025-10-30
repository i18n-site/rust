pub const fn concat_array<const A: usize, const B: usize, const C: usize, T: Copy>(
  a: [T; A],
  b: [T; B],
) -> [T; C] {
  let _ = C - (A + B);
  let _ = (A + B) - C;

  let mut result = std::mem::MaybeUninit::<[T; C]>::uninit();
  let ptr = result.as_mut_ptr() as *mut T;

  let mut i = 0;
  while i < A {
    unsafe {
      ptr.add(i).write(a[i]);
    }
    i += 1;
  }

  while i < A + B {
    unsafe {
      ptr.add(i).write(b[i - A]);
    }
    i += 1;
  }

  unsafe { result.assume_init() }
}

#[macro_export]
macro_rules! concat_array {
($($arr:expr),*) => {
  concat_array!(@concat
    $( [$arr ; $arr.len()] )*
  );
};

(@concat [$a:expr; $a_len:expr]) => {
  $a
};

(@concat [$a:expr; $a_len:expr] [$b:expr; $b_len:expr] $($tail:tt)*) => {
  concat_array!(
    @concat
   [concat_array::<{ $a_len }, { $b_len }, { $a_len + $b_len }, _>($a, $b); $a_len + $b_len]
   $($tail)*
  )
};
}
