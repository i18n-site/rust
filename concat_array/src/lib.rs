pub const fn concat_array<const A: usize, const B: usize, const C: usize>(
  a: [i32; A],
  b: [i32; B],
) -> [i32; C] {
  // Assert that `A + B == C`.
  // These overflow if that is not the case, which produces an error at compile-time.
  let _ = C - (A + B); // Assert that `A + B <= C`
  let _ = (A + B) - C; // Assert that `A + B >= C`

  let mut result = [0; C];

  let mut i = 0;
  while i < A {
    result[i] = a[i];
    i += 1;
  }

  while i < A + B {
    result[i] = b[i - A];
    i += 1;
  }

  result
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
   [concat_array::<{ $a_len }, { $b_len }, { $a_len + $b_len }>($a, $b); $a_len + $b_len]
   $($tail)*
  )
};
}
