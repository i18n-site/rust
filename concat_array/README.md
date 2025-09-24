# concat_array

```rust
use concat_array::concat_array;

const A: [i32; 3] = [1, 2, 3];
const B: [i32; 4] = [4, 5, 6, 7];
const C: [i32; 2] = [8, 9];

// Example usage using function:
const AB: [i32; 7] = concat_array(A, B);

// When concatenating multiple arrays the compiler can't figure out the correct const parameters.
// Using the macro does this for you automatically.
const ABC: &[i32] = &concat_array!(A, B, C);

#[test]
fn main() {
  // Can also be used in non-const contexts
  let ab = concat_array(A, B);
  let abc = concat_array!(A, B, C);

  println!("A   = {:?}", A);
  println!("B   = {:?}", B);
  println!("C   = {:?}", C);
  println!();
  println!("AB  = {:?}", AB);
  println!("ABC = {:?}", ABC);
  println!();
  println!("ab  = {:?}", ab);
  println!("abc = {:?}", abc);

  assert_eq!(AB, [1, 2, 3, 4, 5, 6, 7]);
  assert_eq!(ABC, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

  assert_eq!(AB, ab);
  assert_eq!(ABC, abc);
}
```

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

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
