use std::cmp::min;

use rand::{thread_rng, Rng};

use crate::{
  flag::{FLAG, FLAG_POS},
  random_pos::random_pos,
};

pub const N: usize = 3;

#[derive(Debug)]
pub struct Flag {
  pub pos: usize,
  pub size: u32,
  pub x: u32,
  pub y: u32,
}

fn unicode_n(n: usize) -> Vec<usize> {
  let mut rng = rand::thread_rng();
  let mut result = Vec::with_capacity(n);

  FLAG_POS.with(|flag_pos| {
    let mut flag_pos = flag_pos.borrow_mut();
    for i in 0..n {
      let j = rng.gen_range(i..FLAG.len());
      flag_pos.swap(i, j);
      result.push(flag_pos[i]);
    }
  });

  result
}

pub fn flag_li(width: u32, height: u32) -> ([Flag; N], String) {
  let mut rng = thread_rng();

  let base = min(width, height);

  let mut size_li = Vec::with_capacity(N);

  for _ in 0..N {
    size_li.push(rng.gen_range(base / 10..1 + base / 8));
  }

  let mut size_pos: [Flag; N] =
    unsafe { std::mem::transmute([0u8; std::mem::size_of::<[Flag; N]>()]) };

  let mut svg = Vec::with_capacity(N);
  let ico_pos = unicode_n(N);
  for (pos, (x, y)) in (random_pos(width, height, &size_li[..]))
    .into_iter()
    .enumerate()
  {
    let size = size_li[pos];
    let p = ico_pos[pos];
    svg.push( format!("<svg viewBox=\"0 0 1024 1024\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"><path d=\"{}\" fill=\"url(#ico)\" fill-opacity=\".75\" transform=\"skewX({}) skewY({})\"></path></svg>",
                x, y, size, size, FLAG[p], rng.gen_range(-5..5), rng.gen_range(-5..5)));
    size_pos[pos] = Flag { pos: p, size, x, y };
  }

  (size_pos, svg.join(""))
}
