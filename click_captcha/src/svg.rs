use rand::Rng;

use crate::{
  flag_li::{flag_li, Flag, N},
  pattern::PATTERN,
};

pub fn gen(width: u32, height: u32) -> (String, [Flag; N]) {
  let mut rng = rand::thread_rng();
  let layer_count = random_int(3, 4) as _;
  let segment_count = rng.gen::<u32>() % 10 + 5;
  let wave = Wave::new(Properties {
    width, // 此处的 width 和 height 应该是已定义的变量
    height,
    segment_count,
    layer_count,
    variance: rng.gen_range(0..15) as f32 + 0.1,
  });

  let mut svg = wave.generate_svg();
  let mut opacity = rng.gen_range::<u16, _>(200..300) as f32 / 1000.0;
  let opstep = opacity / layer_count as f32;
  svg.path.reverse();

  let mut path = vec![];

  for (n, i) in svg.path.iter().enumerate() {
    let path_string = format!(
      r#"<path d="{}" stroke-dasharray="{}" stroke="rgba({},{},{},{})" stroke-width="{}px" fill="url(#bg{})" fill-opacity="{}" transform="rotate({} {} {})"></path>"#,
      i.d,
      random_int(30, 0),
      random_int(200, 50),
      random_int(200, 50),
      random_int(200, 50),
      random_int(20, 0) as f32 / 100.0,
      random_int(5, 0),
      n % 4,
      opacity,
      if n % 2 == 0 { 0 } else { 180 },
      width / 2,
      height / 2
    );

    path.push(path_string);
    opacity -= opstep;
  }

  let (flag_li, flag_path) = flag_li(width, height);

  path.insert(path.len() / 2, flag_path);

  let path = path.join("");

  let (psize, pattern) = PATTERN[rng.gen_range(0..PATTERN.len())];
  let mut color = [random_color(90), random_color(180)];

  if rand::random::<u8>() % 2 != 0 {
    color.reverse();
  }
  let ico_scale = random_int(8, 3) as f32 / 100.0;
  let p_scale = random_int(125, 25) as f32 / 25.0;
  let p_rotate = rng.gen::<u16>() % 360;

  let rect_opacity = random_int(20, 10) as f32 / 100.0;

  let xml = format!(
    r###"<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg"><defs><linearGradient id="bg0" x1="50%" y1="0" x2="50%" y2="100%"><stop offset="0%" stop-color="#{}"></stop><stop offset="100%" stop-color="#{}"></stop></linearGradient><linearGradient id="bg1" x1="0%" y1="50%" x2="100%" y2="50%"><stop offset="100%" stop-color="#{}"></stop><stop offset="0%" stop-color="#{}"></stop></linearGradient><linearGradient id="bg2" x1="0%" y1="0" x2="100%" y2="100%"><stop offset="0%" stop-color="#{}"></stop><stop offset="100%" stop-color="#{}"></stop></linearGradient><pattern id="ico" fill="#{}" patternTransform="scale({})" width="1024px" height="1024px" patternUnits="userSpaceOnUse"><path fill="#{}" d="M322.56 400.32c25.6-70.4-18.88-190.4-37.44-234.56a17.28 17.28 0 0 0-24-8.64c-42.56 22.08-153.6 85.44-179.2 155.84a128 128 0 1 0 240.64 87.68z m196.16 32C448 469.44 265.6 573.44 224 689.28A210.56 210.56 0 1 0 619.52 832c42.24-115.84-32-313.28-61.44-385.92a28.48 28.48 0 0 0-39.36-12.8zM893.76 64a21.44 21.44 0 0 0-29.76-9.6c-52.8 27.52-192 105.92-224 192a160 160 0 1 0 298.88 108.8c32.96-86.08-22.4-235.2-45.12-291.2z"></path></pattern><pattern id="p" patternTransform="scale({}) rotate({})" width="{}px" height="{}px" patternUnits="userSpaceOnUse"><path fill="url(#bg2)" d="{}"></path></pattern></defs><rect fill-opacity="{}" height="100%" width="100%" fill="url(#p)"></rect>{}</svg>"###,
    width,
    height,
    color[0],
    color[1],
    color[0],
    color[1],
    color[0],
    color[1],
    random_color(50),
    ico_scale,
    random_color(50),
    p_scale,
    p_rotate,
    psize,
    psize,
    pattern,
    rect_opacity,
    path
  );
  (xml, flag_li)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Properties {
  width: u32,
  height: u32,
  segment_count: u32,
  layer_count: u32,
  variance: f32,
  // fill_color: String,
  // transform: String,
}

#[derive(Clone, Debug)]
pub struct Svg {
  path: Vec<PathAttributes>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wave {
  properties: Properties,
  points: Vec<Vec<Point>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathAttributes {
  // fill: String,
  d: String,
  // transform: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControlPoints {
  pub p1: Vec<f32>,
  pub p2: Vec<f32>,
}

/// Computes control points given knots k.
pub fn compute_control_points(k: &[u32]) -> ControlPoints {
  let k: Vec<_> = k.iter().map(|i| *i as f32).collect();
  let n = k.len() - 1;

  // Initial vectors
  let mut a = vec![0.0; n];
  let mut b = vec![0.0; n];
  let mut c = vec![0.0; n];
  let mut r = vec![0.0; n];
  let mut p1 = vec![0.0; n];
  let mut p2 = vec![0.0; n];

  // Leftmost segment
  a[0] = 0.0;
  b[0] = 2.0;
  c[0] = 1.0;
  r[0] = k[0] + 2.0 * k[1];

  // Internal segments
  for i in 1..n - 1 {
    a[i] = 1.0;
    b[i] = 4.0;
    c[i] = 1.0;
    r[i] = 4.0 * k[i] + 2.0 * k[i + 1];
  }

  // Right segment
  a[n - 1] = 2.0;
  b[n - 1] = 7.0;
  c[n - 1] = 0.0;
  r[n - 1] = 8.0 * k[n - 1] + k[n];

  // Thomas algorithm (from Wikipedia)
  for i in 1..n {
    let m = a[i] / b[i - 1];
    b[i] -= m * c[i - 1];
    r[i] -= m * r[i - 1];
  }

  p1[n - 1] = r[n - 1] / b[n - 1];
  for i in (0..n - 1).rev() {
    p1[i] = (r[i] - c[i] * p1[i + 1]) / b[i];
  }

  // Compute p2
  for i in 0..n - 1 {
    p2[i] = 2.0 * k[i + 1] - p1[i + 1];
  }

  p2[n - 1] = 0.5 * (k[n] + p1[n - 1]);

  ControlPoints { p1, p2 }
}

pub fn generate_closed_path(
  curve_points: &[Point],
  left_corner_point: Point,
  right_corner_point: Point,
) -> PathAttributes {
  let x_points: Vec<_> = curve_points.iter().map(|p| p.x).collect();
  let y_points: Vec<_> = curve_points.iter().map(|p| p.y).collect();

  let x_control_points = compute_control_points(&x_points);
  let y_control_points = compute_control_points(&y_points);

  let mut path = format!(
    "M {},{} C {},{} {},{} {},{} ",
    left_corner_point.x,
    left_corner_point.y,
    left_corner_point.x,
    left_corner_point.y,
    x_points[0],
    y_points[0],
    x_points[0],
    y_points[0]
  );

  for i in 0..x_points.len() - 1 {
    path += &format!(
      "C {},{} {},{} {},{} ",
      x_control_points.p1[i],
      y_control_points.p1[i],
      x_control_points.p2[i],
      y_control_points.p2[i],
      x_points[i + 1],
      y_points[i + 1]
    );
  }

  path += &format!(
    "C {},{} {},{} {},{} Z",
    x_points[x_points.len() - 1],
    y_points[y_points.len() - 1],
    right_corner_point.x,
    right_corner_point.y,
    right_corner_point.x,
    right_corner_point.y
  );

  PathAttributes { d: path }
}

fn random_int(base: u8, offset: u8) -> u8 {
  let n = rand::thread_rng().gen::<u8>() % base;
  n + offset
}

fn random_color(base: u8) -> String {
  let mut i = 0;
  let mut r = Vec::new();
  let mut n = 0.0;
  while i < 3 {
    r.push(random_int(254, 1));
    n += r[i] as f32;
    i += 1;
  }
  n /= 3.0 * base as f32;
  r = r
    .iter()
    .map(|&val| (val as f32 / n).round() as u8)
    .collect();

  format!("{:02x}{:02x}{:02x}", r[0], r[1], r[2])
}

fn generate_points(
  width: u32,
  height: u32,
  segment_count: u32,
  layer_count: u32,
  variance: f32,
) -> Vec<Vec<Point>> {
  // let layer_count = layer_count.unwrap_or(2);
  let cell_width = width as f32 / segment_count as f32;
  let cell_height = height as f32 / layer_count as f32;
  let move_limit_x = cell_width * variance * rand::random::<f32>();
  let move_limit_y = cell_height * variance;

  let mut points: Vec<Vec<Point>> = Vec::new();

  for y in (cell_height as u32..height).step_by(cell_height as _) {
    let mut points_per_layer: Vec<Point> = Vec::new();
    points_per_layer.push(Point { x: 0, y });
    for x in (cell_width as u32..width).step_by(cell_width as _) {
      let varietal_y = y as f32 - move_limit_y / 2.0 + rand::random::<f32>() * move_limit_y;
      let varietal_x = x as f32 - move_limit_x / 2.0 + rand::random::<f32>() * move_limit_x;
      points_per_layer.push(Point {
        x: varietal_x.floor() as _,
        y: varietal_y.floor() as _,
      });
    }
    points_per_layer.push(Point { x: width, y });
    points.push(points_per_layer);
  }
  points
}

impl Wave {
  pub fn new(properties: Properties) -> Self {
    let points = generate_points(
      properties.width,
      properties.height,
      properties.segment_count,
      properties.layer_count,
      properties.variance,
    );

    Wave { properties, points }
  }

  pub fn generate_svg(&self) -> Svg {
    let mut path_list = vec![];

    for point in &self.points {
      let path_data = generate_closed_path(
        &point[..],
        Point {
          x: 0,
          y: self.properties.height,
        },
        Point {
          x: self.properties.width,
          y: self.properties.height,
        },
        // &self.properties.fill_color,
        // &self.properties.transform,
      );
      path_list.push(path_data);
    }

    Svg { path: path_list }
  }
}
