[English](#english) | [中文](#中文)

# English

## Biased Random Number Generation

This Rust library offers a flexible and efficient function for generating non-uniformly distributed random integers. It allows for the creation of random values where the probability distribution is skewed towards the beginning or end of a given range, controlled by a `bias` factor.

The core function, `rng`, is generic and operates on any range implementing `RangeBounds<T>`, making it suitable for various integer types and range specifications (e.g., `0..100` or `0..=100`). This makes it a powerful tool for scenarios like game development (e.g., for weighted loot drops), statistical simulations, or any application requiring a controlled, non-uniform probability distribution.

## Demonstration

The following example demonstrates how to use the `rng` function to generate a distribution of numbers and visualize it as a simple ASCII histogram.

### Example Code

This code generates 10,000 random numbers within the range `0..20`, with a bias factor of 3.0 (favoring smaller numbers), and then prints a histogram of the results.

```rust
use std::collections::BTreeMap;
use biased::rng;

fn main() {
    const N: usize = 20;
    const BIAS: f64 = 3.0;
    const SAMPLES: usize = 10000;

    let mut histogram = BTreeMap::new();
    for _ in 0..SAMPLES {
        // Generate a biased random number within the range 0..20
        let num = rng(0..N, BIAS);
        *histogram.entry(num).or_insert(0) += 1;
    }

    let max_freq = histogram.values().max().cloned().unwrap_or(0);
    const MAX_BAR_WIDTH: usize = 20;

    println!("> ASCII Histogram of the results (range: 0..{}, bias: {}):", N, BIAS);

    for i in 0..N {
        let freq = histogram.get(&i).cloned().unwrap_or(0);
        let bar_width = if max_freq > 0 {
            (freq as usize * MAX_BAR_WIDTH) / max_freq as usize
        } else {
            0
        };
        let bar: String = "█".repeat(bar_width);
        let percentage = (freq as f64 / SAMPLES as f64) * 100.0;
        println!("{:>3}: {:<5} ({:>5.2}%) |{}", i, freq, percentage, bar);
    }
}
```

### Output

The resulting histogram clearly shows that lower numbers appear much more frequently, demonstrating the effect of the bias.

```
> ASCII Histogram of the results (range: 0..20, bias: 3.0):
  0: 3657  (36.57%) |████████████████████
  1: 940   ( 9.40%) |█████
  2: 658   ( 6.58%) |███
  ... (output truncated for brevity)
```

## Design and Technology

### Algorithm

The biasing algorithm is a simple and efficient form of Inverse Transform Sampling:
1.  A uniformly distributed random number `r` is generated in the range `[0.0, 1.0)`.
2.  This number is transformed using a power function: `r.powf(bias)`. When `bias > 1`, this operation "warps" the distribution, pushing most values closer to 0.
3.  The result is scaled from the `[0.0, 1.0)` range to the desired integer offset and added to the range's starting point.
4.  The function intelligently handles different range types (like `a..b` and `a..=b`) by resolving the `RangeBounds` trait into a concrete start and end.

### Tech Stack

-   **Language:** Rust (2024 Edition)
-   **Core Dependencies:**
    -   `rand`: For the underlying uniform random number generation.
    -   `num-traits`: To provide generic numeric capabilities for handling various integer types.

The project is designed with a focus on flexibility, safety, and minimal dependencies, adhering to modern Rust idioms.

## Project Structure

The project is organized as follows:

```
.
├── Cargo.toml      # Package manifest and dependencies
├── README.md       # The compiled README file
├── README.mdt      # The source template for the README
├── test.sh         # Script to run all tests and checks
├── src
│   └── lib.rs      # The core library code for the `rng` function
└── tests
    └── main.rs     # Integration tests and histogram generation example
```

## A Brief History of Randomness

The quest for high-quality random numbers is as old as computing itself. In the 1940s, during the Manhattan Project, scientists like John von Neumann required vast quantities of random numbers for their Monte Carlo simulations of neutron chain reactions. Initially, they used mechanical devices and published tables of random digits. However, these methods were slow and cumbersome.

Von Neumann proposed the "middle-square" method for generating pseudo-random numbers algorithmically: take a number, square it, and extract the middle digits as the next number. While innovative, it had critical flaws, such as a tendency to fall into short, repeating cycles. This early work highlighted the profound difficulty of making a deterministic machine produce something that appears truly random, paving the way for the sophisticated, statistically robust pseudo-random number generators (PRNGs) we rely on today in fields from cryptography to, well, biased random number generation in Rust!

---

# 中文

## 偏向性随机数生成

本 Rust 库提供了一个灵活而高效的函数，用于生成非均匀分布的随机整数。它允许创建的随机值概率分布偏向给定范围的起始或末尾，这由一个 `bias`（偏向）因子控制。

核心函数 `rng` 是泛型的，可对任何实现了 `RangeBounds<T>` 的范围进行操作，使其适用于各种整数类型和范围规范（例如 `0..100` 或 `0..=100`）。这使其成为一个强大的工具，适用于游戏开发（例如，加权战利品掉落）、统计模拟或任何需要受控的非均匀概率分布的应用场景。

## 功能演示

以下示例演示了如何使用 `rng` 函数生成数字分布，并将其可视化为一个简单的 ASCII 直方图。

### 示例代码

此代码生成 10,000 个在 `0..20` 范围内的随机数，偏向因子为 3.0（偏向较小的数字），然后打印结果的直方图。

```rust
use std::collections::BTreeMap;
use biased::rng;

fn main() {
    const N: usize = 20;
    const BIAS: f64 = 3.0;
    const SAMPLES: usize = 10000;

    let mut histogram = BTreeMap::new();
    for _ in 0..SAMPLES {
        // 在 0..20 的范围内生成一个偏向性随机数
        let num = rng(0..N, BIAS);
        *histogram.entry(num).or_insert(0) += 1;
    }

    let max_freq = histogram.values().max().cloned().unwrap_or(0);
    const MAX_BAR_WIDTH: usize = 20;

    println!("> ASCII 直方图结果 (范围: 0..{}, bias: {}):", N, BIAS);

    for i in 0..N {
        let freq = histogram.get(&i).cloned().unwrap_or(0);
        let bar_width = if max_freq > 0 {
            (freq as usize * MAX_BAR_WIDTH) / max_freq as usize
        } else {
            0
        };
        let bar: String = "█".repeat(bar_width);
        let percentage = (freq as f64 / SAMPLES as f64) * 100.0;
        println!("{:>3}: {:<5} ({:>5.2}%) |{}", i, freq, percentage, bar);
    }
}
```

### 输出

生成的直方图清晰地显示了较小的数字出现得更频繁，证明了偏向因子的效果。

```
> ASCII 直方图结果 (范围: 0..20, bias: 3.0):
  0: 3657  (36.57%) |████████████████████
  1: 940   ( 9.40%) |█████
  2: 658   ( 6.58%) |███
  ... (为简洁起见，输出已截断)
```

## 设计与技术

### 算法

偏向性算法是“逆变换采样”的一种简单而高效的形式：
1.  在 `[0.0, 1.0)` 范围内生成一个均匀分布的随机数 `r`。
2.  使用幂函数 `r.powf(bias)` 对该数字进行变换。当 `bias > 1` 时，此操作会“扭曲”分布，将更多值推向 0。
3.  将结果从 `[0.0, 1.0)` 范围缩放到所需的目标整数偏移量，并将其加到范围的起始点上。
4.  函数通过解析 `RangeBounds` trait，可以智能地处理不同的范围类型（如 `a..b` 和 `a..=b`）。

### 技术栈

-   **语言:** Rust (2024 Edition)
-   **核心依赖:**
    -   `rand`: 用于底层的均匀随机数生成。
    -   `num-traits`: 提供泛型数值能力，以处理各种整数类型。

该项目的设计注重灵活性、安全性及最小化依赖，并遵循现代 Rust 的习惯用法。

## 项目结构

项目结构组织如下：

```
.
├── Cargo.toml      # 包清单与依赖项
├── README.md       # 编译后的 README 文件
├── README.mdt      # README 的源模板
├── test.sh         # 用于运行所有测试和检查的脚本
├── src
│   └── lib.rs      # `rng` 函数的核心库代码
└── tests
    └── main.rs     # 集成测试和直方图生成示例
```

## 随机性的简史

对高质量随机数的追求与计算本身一样古老。在 1940 年代的曼哈顿计划期间，像约翰·冯·诺伊曼这样的科学家需要大量的随机数来进行中子链式反应的蒙特卡洛模拟。最初，他们使用机械设备和已出版的随机数字表。然而，这些方法缓慢且笨拙。

冯·诺伊曼提出了用于算法生成伪随机数的“平方取中法”：取一个数，将其平方，然后提取中间的数字作为下一个数。虽然这个方法很有创意，但它存在严重缺陷，比如容易陷入简短的重复循环。这项早期工作凸显了让确定性机器产生看起来真正随机的东西的巨大困难，为我们今天在密码学乃至 Rust 中的偏向性随机数生成等领域所依赖的、统计上稳健的伪随机数生成器（PRNG）铺平了道路。

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 : MarkDown 命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖 （ 如果这段原文没有被修改 ）。

* [i18n.site : MarkDown 多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
