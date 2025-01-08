use std::cmp::min;

use rand::{seq::index::sample, thread_rng, Rng};

use crate::random_pos::random_pos;

pub const N: usize = 3;

#[derive(Debug)]
pub struct Flag {
  pub pos: usize,
  pub size: u32,
  pub x: u32,
  pub y: u32,
}

pub fn flag<S: AsRef<str>>(
  width: u32,
  height: u32,
  ico_li: impl AsRef<[S]>,
) -> ([Flag; N], String) {
  let mut rng = thread_rng();

  let ico_li = ico_li.as_ref();

  let base = min(width, height);

  let mut size_li = Vec::with_capacity(N);

  for _ in 0..N {
    size_li.push(rng.gen_range(base / 10..1 + base / 8));
  }

  let mut size_pos: [Flag; N] =
    unsafe { std::mem::transmute([0u8; std::mem::size_of::<[Flag; N]>()]) };

  let mut svg = Vec::with_capacity(N);

  for ((pos, (x, y)), p) in (random_pos(width, height, &size_li[..]))
    .into_iter()
    .enumerate()
    .zip(sample(&mut rng, ico_li.len(), N))
  {
    let size = size_li[pos];
    svg.push(
      format!("<svg viewBox=\"0 0 1024 1024\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"><path d=\"{}\" fill=\"url(#ico)\" fill-opacity=\".75\" transform=\"skewX({}) skewY({})\"></path></svg>",
                x, y, size, size, ico_li[p].as_ref(), rng.gen_range(-5..5), rng.gen_range(-5..5)));
    size_pos[pos] = Flag { pos: p, size, x, y };
  }

  (size_pos, svg.join(""))
}
