use concat_array::concat_array;

const A: [i32; 3] = [1, 2, 3];
const B: [i32; 4] = [4, 5, 6, 7];
const C: [i32; 2] = [8, 9];

// Example usage using function:
const AB: [i32; 7] = concat_array(A, B);

// When concatenating multiple arrays the compiler can't figure out the correct const parameters.
// Using the macro does this for you automatically.
const ABC: &[i32] = &concat_array!(A, B, C);

#[test]
fn main() {
  // Can also be used in non-const contexts
  let ab = concat_array(A, B);
  let abc = concat_array!(A, B, C);

  println!("A   = {:?}", A);
  println!("B   = {:?}", B);
  println!("C   = {:?}", C);
  println!();
  println!("AB  = {:?}", AB);
  println!("ABC = {:?}", ABC);
  println!();
  println!("ab  = {:?}", ab);
  println!("abc = {:?}", abc);

  assert_eq!(AB, [1, 2, 3, 4, 5, 6, 7]);
  assert_eq!(ABC, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

  assert_eq!(AB, ab);
  assert_eq!(ABC, abc);
}
