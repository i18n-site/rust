# zset

* [English](#english)
* [中文](#中文)

`zset` is a thread-safe sorted set data structure inspired by Redis's zset. It allows for efficient storage and retrieval of unique members, each associated with a score. Members are sorted based on their scores, enabling fast rank and range queries.

This implementation is designed for high-concurrency scenarios, utilizing lock-free and fine-grained locking techniques to minimize contention and maximize performance in multi-threaded applications.

`zset` 是一个受 Redis zset 启发的线程安全排序集数据结构。它允许高效地存储和检索唯一的成员，每个成员都与一个分数相关联。成员根据其分数进行排序，从而实现快速的排名和范围查询。

此实现专为高并发场景设计，利用无锁和细粒度锁定技术，最大限度地减少多线程应用中的竞争并提升性能。

---

## English

* [Overview](#overview)
* [Features](#features)
* [Design and Tech Stack](#design-and-tech-stack)
* [Usage](#usage)
* [File Structure](#file-structure)
* [A Little Story](#a-little-story)

### Overview

A sorted set is a hybrid data structure that combines the features of a hash map and a sorted list. It maps members to scores, ensuring that each member is unique, while also maintaining a sorted order of members based on their scores. This makes it an ideal choice for applications that require both fast lookups (by member) and ordered traversal (by score), such as leaderboards, real-time analytics, and priority queues.

This `zset` library provides a generic, thread-safe implementation that can be used with any data types for members and scores that satisfy the required traits.

### Features

*   **Thread-Safe**: Designed for concurrent use across multiple threads with minimal contention.
*   **High Performance**: Utilizes efficient data structures to provide fast operations.
*   **Generic**: Works with any member and score types that implement `Ord`, `Hash`, `Send`, `Sync`, etc.
*   **Rich API**: Offers a comprehensive set of operations, including adding, removing, querying scores and ranks, and retrieving ranges.

### Design and Tech Stack

The design of `zset` revolves around two core data structures to achieve both fast lookups and efficient sorting:

1.  **`DashMap<Arc<M>, S>`**: A concurrent hash map used for O(1) average-time complexity lookups of a member's score. `DashMap` is chosen for its excellent performance in highly contended, multi-threaded environments. It shards the map internally, allowing different threads to access different shards concurrently without blocking each other.

2.  **`RwLock<SortedVec<ScoreMember<M, S>>>`**: A `SortedVec` protected by a `RwLock` is used to maintain the sorted order of members. `SortedVec` is a vector-based data structure that keeps its elements sorted, offering a compromise between fast random access and efficient insertions/deletions with O(sqrt(N)) complexity. The `RwLock` allows multiple threads to read the sorted data concurrently, while ensuring exclusive access for write operations.

This dual-structure approach provides a balanced performance profile:

*   **Score & Member Lookup**: O(1) on average, thanks to `DashMap`.
*   **Rank Lookup**: O(log N), by performing a binary search on the `SortedVec`.
*   **Add/Remove Operations**: O(sqrt(N)), dominated by the `SortedVec`'s insertion/deletion time.
*   **Range Queries**: O(K) where K is the size of the range, as it involves iterating over a slice of the `SortedVec`.

### Usage

Here are some basic usage examples:

**Adding members and getting cardinality:**

```rust
use zset::{Api, Zset};

let zset = Zset::<&str, i32>::new();
// Add a new member, returns false as it's a new element
assert!(!zset.add("one", 1));
assert_eq!(zset.card(), 1);

// Add another new member
assert!(!zset.add("two", 2));
assert_eq!(zset.card(), 2);

// Update the score of an existing member, returns true
assert!(zset.add("one", 10));
assert_eq!(zset.card(), 2);
```

**Querying score and rank:**

```rust
use zset::{Api, Zset};

let zset = Zset::new();
zset.add("one", 10);
zset.add("two", 20);
zset.add("three", 30);

// Get the score of a member
assert_eq!(zset.score(&"two"), Some(20));

// Get the 0-based rank of a member (sorted from low to high score)
assert_eq!(zset.rank(&"one"), Some(0));
assert_eq!(zset.rank(&"two"), Some(1));
assert_eq!(zset.rank(&"three"), Some(2));
```

### File Structure

```
.
├── Cargo.toml       # Package manifest
├── README.mdt       # Readme template file
├── src
│   ├── lib.rs         # Main library file, defines the Api trait
│   ├── score_member.rs # Defines the internal ScoreMember struct
│   └── zset_impl.rs   # The core implementation of the Zset
└── tests
    └── main.rs        # Integration tests
```

### A Little Story

The concept of the sorted set was popularized by Redis, an in-memory data structure store created by Salvatore Sanfilippo. The story goes that Salvatore, nicknamed "antirez," needed a fast way to handle real-time analytics for a web startup. He found that existing databases were too slow for the kind of operations he needed, like finding the top N items in a list that was constantly changing. This led him to create Redis and, within it, the versatile `zset` data structure.

The beauty of the `zset` lies in its dual nature. In Redis, it's implemented using a combination of a hash table and a skip list. The hash table provides fast access to scores, while the skip list keeps the members sorted. This allows Redis to perform complex operations like range queries by score or rank with remarkable speed. Our Rust `zset` follows this inspiration, using modern, concurrent Rust data structures to achieve a similar blend of performance and functionality, making it a powerful tool for building responsive, data-intensive applications.

---

## 中文

* [概览](#概览)
* [特性](#特性)
* [设计与技术栈](#设计与技术栈)
* [使用示例](#使用示例)
* [文件结构](#文件结构-1)
* [一个小故事](#一个小故事)

### 概览

排序集（Sorted Set）是一种混合数据结构，它结合了哈希映射和有序列表的特性。它将成员映射到分数，确保每个成员的唯一性，同时根据分数维护成员的有序排列。这使得它成为需要快速查找（按成员）和有序遍历（按分数）的应用的理想选择，例如排行榜、实时分析和优先级队列。

本 `zset` 库提供了一个通用的、线程安全的实现，可用于任何满足所需 trait 的成员和分数数据类型。

### 特性

*   **线程安全**：专为多线程并发使用而设计，竞争开销极小。
*   **高性能**：利用高效的数据结构提供快速的操作。
*   **通用性**：适用于任何实现了 `Ord`、`Hash`、`Send`、`Sync` 等 trait 的成员和分数类型。
*   **丰富的 API**：提供全面的操作集，包括添加、删除、查询分数和排名以及检索范围。

### 设计与技术栈

`zset` 的设计围绕两个核心数据结构，以实现快速查找和高效排序：

1.  **`DashMap<Arc<M>, S>`**：一个并发哈希映射，用于以 O(1) 的平均时间复杂度查找成员的分数。选择 `DashMap` 是因为它在高度竞争的多线程环境中表现出色。它在内部分片，允许不同线程并发访问不同分片而不会相互阻塞。

2.  **`RwLock<SortedVec<ScoreMember<M, S>>>`**：一个由 `RwLock` 保护的 `SortedVec`，用于维护成员的有序排列。`SortedVec` 是一个基于向量的数据结构，它使其元素保持排序，通过 O(sqrt(N)) 复杂度的插入/删除操作，在快速随机访问和高效修改之间取得了平衡。`RwLock` 允许多个线程并发读取排序后的数据，同时确保写操作的独占访问。

这种双重结构的方法提供了均衡的性能特征：

*   **分数和成员查找**：平均 O(1)，得益于 `DashMap`。
*   **排名查找**：O(log N)，通过在 `SortedVec` 上执行二分搜索实现。
*   **添加/删除操作**：O(sqrt(N))，主要开销在于 `SortedVec` 的插入/删除时间。
*   **范围查询**：O(K)，其中 K 是范围的大小，因为它涉及遍历 `SortedVec` 的一个切片。

### 使用示例

以下是一些基本的使用示例：

**添加成员和获取基数：**

```rust
use zset::{Api, Zset};

let zset = Zset::<&str, i32>::new();
// 添加一个新成员，返回 false 因为它是一个新元素
assert!(!zset.add("one", 1));
assert_eq!(zset.card(), 1);

// 添加另一个新成员
assert!(!zset.add("two", 2));
assert_eq!(zset.card(), 2);

// 更新一个已存在成员的分数，返回 true
assert!(zset.add("one", 10));
assert_eq!(zset.card(), 2);
```

**查询分数和排名：**

```rust
use zset::{Api, Zset};

let zset = Zset::new();
zset.add("one", 10);
zset.add("two", 20);
zset.add("three", 30);

// 获取成员的分数
assert_eq!(zset.score(&"two"), Some(20));

// 获取成员的排名（0-based），按分数从低到高排序
assert_eq!(zset.rank(&"one"), Some(0));
assert_eq!(zset.rank(&"two"), Some(1));
assert_eq!(zset.rank(&"three"), Some(2));
```

### 文件结构

```
.
├── Cargo.toml       # 包清单文件
├── README.mdt       # Readme 模板文件
├── src
│   ├── lib.rs         # 主库文件，定义了 Api trait
│   ├── score_member.rs # 定义了内部的 ScoreMember 结构体
│   └── zset_impl.rs   # Zset 的核心实现
└── tests
    └── main.rs        # 集成测试
```

### 一个小故事

排序集（Sorted Set）的概念由内存数据结构存储 Redis 的创始人 Salvatore Sanfilippo 推广开来。据说，Salvatore（昵称 "antirez"）当时需要一种快速的方法来为一家网络创业公司处理实时分析。他发现现有的数据库对于他需要的操作来说太慢了，比如在一个不断变化的列表中查找前 N 个项目。这促使他创建了 Redis，并在其中设计了功能多样的 `zset` 数据结构。

`zset` 的精妙之处在于其双重性。在 Redis 中，它由哈希表和跳表（Skip List）结合实现。哈希表提供对分数的快速访问，而跳表则保持成员的有序性。这使得 Redis 能够以惊人的速度执行复杂的操作，如按分数或排名进行范围查询。我们的 Rust `zset` 遵循了这一灵感，使用现代的、并发的 Rust 数据结构来实现类似的性能和功能融合，使其成为构建响应迅速、数据密集型应用的强大工具。

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