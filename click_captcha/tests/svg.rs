use std::{collections::HashSet, env, fs, io::Write};

use click_captcha::{gen, svg::gen as svg_gen};

#[test]
fn test_svg() -> anyhow::Result<()> {
  let current_exe_path = env::current_exe().unwrap();
  let target_dir = current_exe_path
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .join("img");

  println!("img → {}", target_dir.to_string_lossy());
  // 2. 检查并创建目录
  if !target_dir.exists() {
    fs::create_dir(&target_dir).unwrap();
  }

  // 3. 写入文件
  for i in 0..10 {
    let file_path = target_dir.join(format!("{i}.svg"));
    let mut file = fs::File::create(file_path).unwrap();
    let g = svg_gen(500, 500);
    file.write_all(g.0.as_bytes()).unwrap();
    let mut idset = HashSet::new();
    for i in &g.1 {
      idset.insert(i.pos);
    }
    assert_eq!(idset.len(), 3);

    println!("{i} {:?}", g.1);
  }
  for i in 10..100 {
    let file_path = target_dir.join(format!("{i}.webp"));
    let mut file = fs::File::create(file_path).unwrap();
    let g = gen(500, 500)?;
    file.write_all(&g.0).unwrap();

    println!("{i} {:?}", g.1);
  }
  Ok(())
}
