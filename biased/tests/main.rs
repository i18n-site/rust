use std::collections::BTreeMap;

use anyhow::Result;
use biased::rng;
use log::info;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
  log_init::init();
}

#[test]
fn test_rng_histogram() -> Result<()> {
  // 上限
  // Upper bound
  const N: u32 = 30;
  // 偏向因子
  // Bias factor
  const BIAS: f64 = 1.8;
  // 采样次数
  // Number of samples
  const SAMPLES: u32 = 10000;

  info!(
    "> Generating {} biased random numbers (n={}, bias={})",
    SAMPLES, N, BIAS
  );

  let mut histogram = BTreeMap::new();
  for _ in 0..SAMPLES {
    let num = rng(0..N, BIAS);
    *histogram.entry(num).or_insert(0) += 1;
  }

  // 找到最大频率用于缩放直方图的条形
  // Find the maximum frequency to scale the histogram bars
  let max_freq = histogram.values().max().cloned().unwrap_or(0);
  const MAX_BAR_WIDTH: usize = 20;

  info!("> ASCII Histogram of the results:");

  for i in 0..N {
    let freq = histogram.get(&i).cloned().unwrap_or(0);
    let bar_width = if max_freq > 0 {
      (freq as usize * MAX_BAR_WIDTH) / max_freq as usize
    } else {
      0
    };
    let bar: String = "█".repeat(bar_width);
    let percentage = (freq as f64 / SAMPLES as f64) * 100.0;
    // 使用 info! 打印，与现有测试保持一致。
    // Use info! to print, as the existing test does.
    println!("{:>3}: {:<5} ({:>5.2}%) |{}", i, freq, percentage, bar);
  }

  Ok(())
}
