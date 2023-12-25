# sts : shortcut for timestamp

[→ tests/main.rs](tests/main.rs)

```rust
use sts::{ms, sec};

#[test]
fn main() {
  dbg!(sec());
  dbg!(ms());
}
```


run tests :

[→ out.txt](out.txt)

```txt
+ cargo test -- --nocapture
   Compiling libc v0.2.147
   Compiling once_cell v1.18.0
   Compiling coarsetime v0.1.23
   Compiling sts v0.0.3 (/Users/z/art/sts)
    Finished test [unoptimized + debuginfo] target(s) in 0.45s
     Running unittests src/lib.rs (target/debug/deps/sts-313e1f24663d0a18)
     Running tests/main.rs (target/debug/deps/main-7ad98a7b1f67db68)
[tests/main.rs:5] sec() = 1692364903
[tests/main.rs:6] ms() = 1692364903773
   Doc-tests sts
```

