use std::cmp::min;

use rand::{thread_rng, Rng};

use crate::{
  flag::{FLAG, FLAG_POS},
  random_pos::random_pos,
};

const N: usize = 3;

fn unicode_n(n: usize) -> Vec<usize> {
  let mut rng = rand::thread_rng();
  let mut result = Vec::with_capacity(n);

  for i in 0..n {
    let j = rng.gen_range(i..FLAG.len());
    unsafe {
      std::mem::swap(&mut FLAG_POS[i], &mut FLAG_POS[j]);
    }
    result.push(unsafe { FLAG_POS[i] });
  }

  result
}

pub fn flag_li(width: u32, height: u32) -> (Vec<[u32; 3]>, String) {
  let mut rng = thread_rng();

  let base = min(width, height);

  let mut size_li = Vec::with_capacity(N);

  for _ in 0..N {
    size_li.push(rng.gen_range(base / 15..1 + base / 10));
  }

  let mut size_pos = Vec::with_capacity(N);
  let mut svg = Vec::with_capacity(N);
  let ico_pos = unicode_n(N);
  for (pos, (x, y)) in (random_pos(width, height, &size_li[..]))
    .into_iter()
    .enumerate()
  {
    let size = size_li[pos];
    svg.push( format!("<svg viewBox=\"0 0 1024 1024\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"><path d=\"{}\" fill=\"url(#ico)\" fill-opacity=\".65\" transform=\"skewX({}) skewY({})\"></path></svg>",
            x, y, size, size, FLAG[ico_pos[pos]], rng.gen_range(-5..5), rng.gen_range(-5..5)));
    size_pos.push([size, x, y])
  }
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

  (size_pos, svg.join(""))
}
