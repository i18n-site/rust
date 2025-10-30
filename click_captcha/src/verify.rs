use crate::PosLi;

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
  ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub fn verify(pos_li: PosLi, click_x_y: &[u32]) -> bool {
  if click_x_y.len() < 2 * pos_li.len() {
    return false;
  }

  for (pos, xys) in pos_li.iter().enumerate() {
    let pos = pos * 2;
    let cx = (click_x_y[pos]) as f32;
    let cy = (click_x_y[pos + 1]) as f32;

    // 起点 + 半径 = 圆心
    let size = (xys.size as f32) / 2.0;

    let d = distance(xys.x as f32 + size, xys.y as f32 + size, cx, cy);

    if d > size {
      return false;
    }
  }
  true
}
