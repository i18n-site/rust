use rand::{thread_rng, Rng};

use crate::flag::FLAG;

pub fn flag_li(height: u32) -> (Vec<[u32; 3]>, String) {
  let mut rng = thread_rng();
  let size = rng.gen_range(height / 20..1 + height / 10);
  let box_x = rng.gen_range(0..height - size);
  let box_y = rng.gen_range(0..height - size);
  let n = rng.gen_range(0..FLAG.len());
  let svg = format!("<svg viewBox=\"0 0 1024 1024\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"><path d=\"{}\" fill=\"url(#ico)\" fill-opacity=\".65\" transform=\"skewX({}) skewY({})\"></path></svg>",
        box_x, box_y, size, size, FLAG[n], rng.gen_range(-10..10), rng.gen_range(-10..10));
  //
  // path.insert(layer_count / 2, svg);
  //
  // let rect = format!(
  //   "<rect fill-opacity=\"{}\" height=\"100%\" width=\"100%\" fill=\"url(#bg2)\"></rect>",
  //   random_int(0, 30) as f32 / 100.0
  // );
  //
  // path.push(rect);
  //
  // // For demonstration purposes, print the path vector
  // for item in &path {
  //   println!("{}", item);
  // }

  (vec![[box_x, box_y, size]], svg)
}
