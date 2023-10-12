use rand::Rng;

mod pattern;

#[derive(Clone, Debug, PartialEq)]
pub struct Properties {
  width: usize,
  height: usize,
  segment_count: usize,
  layer_count: usize,
  variance: f32,
  fill_color: String,
  stroke_color: String,
  stroke_width: f32,
  transform: String,
}

#[derive(Clone, Debug)]
pub struct Svg {
  width: usize,
  height: usize,
  xmlns: String,
  path: Vec<PathAttributes>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wavery {
  properties: Properties,
  points: Vec<Vec<Point>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathAttributes {
  fill: String,
  stroke_color: String,
  stroke_width: f32,
  d: String,
  transform: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControlPoints {
  pub p1: Vec<f32>,
  pub p2: Vec<f32>,
}

/// Computes control points given knots k.
pub fn compute_control_points(k: &[usize]) -> ControlPoints {
  let k: Vec<_> = k.into_iter().map(|i| *i as f32).collect();
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

const SVGNS: &str = "http://www.w3.org/2000/svg";

pub fn generate_closed_path(
  curve_points: &[Point],
  left_corner_point: Point,
  right_corner_point: Point,
  fill_color: &str,
  stroke_color: &str,
  stroke_width: f32,
  transform: &str,
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

  PathAttributes {
    fill: fill_color.to_string(),
    stroke_color: stroke_color.to_string(),
    stroke_width,
    d: path,
    transform: transform.to_string(),
  }
}

fn random_int(base: u8, offset: u8) -> u8 {
  let rng = rand::thread_rng();
  let n = rng.gen::<u8>() % base;
  n + offset
}

fn random_color(base: u8) -> String {
  let mut i = 0;
  let mut r = Vec::new();
  let mut n = 0.0;
  while i < 3 {
    r.push(random_int(255, 0));
    n += r[i] as f32;
    i += 1;
  }
  n = n / (3.0 * base as f32);
  r = r
    .iter()
    .map(|&val| {
      let mut new_val = (val as f64 / n).round() as u8;
      if new_val > 255 {
        new_val = 255;
      }
      new_val
    })
    .collect();

  format!("{:02x}{:02x}{:02x}", r[0], r[1], r[2])
}

fn compute_animated_path(
  points: &Vec<Point>,
  left_corner_point: &Point,
  right_corner_point: &Point,
  x_points: &[usize],
) -> String {
  let ani_x_points: Vec<_> = points.iter().map(|p| p.x).collect();
  let ani_y_points: Vec<_> = points.iter().map(|p| p.y).collect();

  let ani_x_control_points = compute_control_points(&ani_x_points);
  let ani_y_control_points = compute_control_points(&ani_y_points);

  let mut animated_path = format!(
    "M {},{} C {},{} {},{} {},{} ",
    left_corner_point.x,
    left_corner_point.y,
    left_corner_point.x,
    left_corner_point.y,
    ani_x_points[0],
    ani_y_points[0],
    ani_x_points[0],
    ani_y_points[0]
  );

  for i in 0..x_points.len() - 1 {
    animated_path += &format!(
      "C {},{} {},{} {},{} ",
      ani_x_control_points.p1[i],
      ani_y_control_points.p1[i],
      ani_x_control_points.p2[i],
      ani_y_control_points.p2[i],
      ani_x_points[i + 1],
      ani_y_points[i + 1]
    );
  }

  animated_path += &format!(
    "C {},{} {},{} {},{} Z",
    ani_x_points[x_points.len() - 1],
    ani_y_points[x_points.len() - 1],
    right_corner_point.x,
    right_corner_point.y,
    right_corner_point.x,
    right_corner_point.y
  );

  animated_path
}

fn generate_points(
  width: usize,
  height: usize,
  segment_count: usize,
  layer_count: usize,
  variance: f32,
) -> Vec<Vec<Point>> {
  // let layer_count = layer_count.unwrap_or(2);
  let cell_width = width as f32 / segment_count as f32;
  let cell_height = height as f32 / layer_count as f32;
  let move_limit_x = cell_width * variance * rand::random::<f32>();
  let move_limit_y = cell_height * variance;

  let mut points: Vec<Vec<Point>> = Vec::new();

  for y in (cell_height as usize..height as usize).step_by(cell_height as usize) {
    let mut points_per_layer: Vec<Point> = Vec::new();
    points_per_layer.push(Point { x: 0, y });
    for x in (cell_width as usize..width as usize).step_by(cell_width as usize) {
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

impl Wavery {
  pub fn new(properties: Properties) -> Self {
    let points = generate_points(
      properties.width,
      properties.height,
      properties.segment_count,
      properties.layer_count,
      properties.variance,
    );

    Wavery { properties, points }
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
        &self.properties.fill_color,
        &self.properties.stroke_color,
        self.properties.stroke_width,
        &self.properties.transform,
      );
      path_list.push(path_data);
    }

    Svg {
      width: self.properties.width,
      height: self.properties.height,
      xmlns: SVGNS.to_string(),
      path: path_list,
    }
  }
}
