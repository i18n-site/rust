use std::cmp::min;

use rand::{Rng, SeedableRng, rngs::StdRng, seq::index::sample};

use crate::{N, Pos, PosLi, random_pos::random_pos};

#[derive(Debug)]
pub struct IcoPosLi {
  pub ico_li: [usize; N],
  pub pos_li: PosLi,
}

pub fn ico<S: AsRef<str>>(width: u32, height: u32, ico_li: impl AsRef<[S]>) -> (IcoPosLi, String) {
  let mut rng = StdRng::from_os_rng();

  let ico_li = ico_li.as_ref();

  let base = min(width, height);

  let mut size_li = Vec::with_capacity(N);

  for _ in 0..N {
    size_li.push(rng.random_range(base / 10..1 + base / 8));
  }

  let mut ipl: IcoPosLi = unsafe { std::mem::transmute([0u8; std::mem::size_of::<IcoPosLi>()]) };

  let mut svg = Vec::with_capacity(N);

  for ((pos, (x, y)), ico) in (random_pos(width, height, &size_li[..]))
    .into_iter()
    .enumerate()
    .zip(sample(&mut rng, ico_li.len(), N))
  {
    let size = size_li[pos];
    svg.push(
      format!("<svg viewBox=\"0 0 1024 1024\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"><path d=\"{}\" fill=\"url(#ico)\" fill-opacity=\".75\" transform=\"skewX({}) skewY({})\"></path></svg>",
                x, y, size, size, ico_li[ico].as_ref(), rng.random_range(-5..5), rng.random_range(-5..5)));

    ipl.ico_li[pos] = ico;
    ipl.pos_li[pos] = Pos { size, x, y };
  }

  (ipl, svg.join(""))
}
