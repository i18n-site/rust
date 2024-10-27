use daachorse::errors::Result;
use varurl::VarUrl;

#[test]
fn test_complex_markdown() -> Result<()> {
  let patterns = vec!["https://i18n-img.github.io/", "https://example.com/", "/"];
  let var_url = VarUrl::new(patterns, "en")?;

  let md = r#"# Main Title

## Section 1
Here's a paragraph with an ![inline image](https://i18n-img.github.io/en/inline.png) and a [link](https://i18n-img.github.io/en/doc.md) mixed in.
"#;

  let mdli = mdli::md_parse(md);
  let result = var_url.replace(&mdli, "zh");

  // 添加调试输出
  dbg!(&result);
  dbg!(mdli);

  // assert!(result.contains("![inline image](https://i18n-img.github.io/zh/inline.png)"));

  println!("\n-----\n{}", result);
  Ok(())
}

// #[test]
// fn test_var_url() -> Result<()> {
//   let patterns = vec![
//     "https://i18n-img.github.io/",
//     "/", // 用于相对路径
//   ];
//   let var_url = VarUrl::new(patterns, "en")?;
//
//   // 测试 Markdown 图片
//   let md = "![xx](https://i18n-img.github.io/en/1.avif)";
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     "![xx](https://i18n-img.github.io/zh/1.avif)"
//   );
//
//   // 测试 HTML video 标签
//   let md = r#"<video src="https://i18n-img.github.io/en/1.mp4"></video>"#;
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     r#"<video src="https://i18n-img.github.io/zh/1.mp4"></video>"#
//   );
//
//   // 测试 Markdown 链接
//   let md = "[xx](https://i18n-img.github.io/en/README.md)";
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     "[xx](https://i18n-img.github.io/zh/README.md)"
//   );
//
//   // 测试 HTML a 标签
//   let md = r#"<a style="color:red" href="https://i18n-img.github.io/en/i18n.site.gz">xx</a>"#;
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     r#"<a style="color:red" href="https://i18n-img.github.io/zh/i18n.site.gz">xx</a>"#
//   );
//
//   // 测试相对路径
//   let md = "![xx](/en/1.avif)";
//   let mdli = mdli::md_parse(md);
//   assert_eq!(var_url.replace(&mdli, "zh"), "![xx](/zh/1.avif)");
//
//   // 测试不应该替换的情况
//   let md = "![xx](//i18n-img.github.io/en/1.avif)"; // 以 // 开头
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     "![xx](//i18n-img.github.io/en/1.avif)"
//   );
//
//   // 测试普通文本中的 URL（不应该替换）
//   let md = "Visit https://i18n-img.github.io/en/1.avif for more info";
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     "Visit https://i18n-img.github.io/en/1.avif for more info"
//   );
//
//   // 测试混合内容
//   let md = r#"# Title
// ![img1](https://i18n-img.github.io/en/1.avif)
// Some text here
// <video src="https://i18n-img.github.io/en/2.mp4"></video>
// [link](/en/doc.md)
// <a href="https://i18n-img.github.io/en/page.html">link</a>"#;
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     r#"# Title
// ![img1](https://i18n-img.github.io/zh/1.avif)
// Some text here
// <video src="https://i18n-img.github.io/zh/2.mp4"></video>
// [link](/zh/doc.md)
// <a href="https://i18n-img.github.io/zh/page.html">link</a>"#
//   );
//
//   Ok(())
// }
//
// #[test]
// fn test_invalid_patterns() {
//   // 测试无效的模式
//   let patterns: Vec<String> = vec![];
//   assert!(VarUrl::new(patterns, "en").is_err());
// }
//
// #[test]
// fn test_edge_cases() -> Result<()> {
//   let patterns = vec!["https://i18n-img.github.io/"];
//   let var_url = VarUrl::new(patterns, "en")?;
//
//   // 测试不完整的 URL
//   let md = "![xx](https://i18n-img.github.io/en)"; // 没有结尾的 /
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     "![xx](https://i18n-img.github.io/en)"
//   );
//
//   // 测试错误的语言代码格式
//   let md = "![xx](https://i18n-img.github.io/en-US/1.avif)"; // 不是简单的两字符代码
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     "![xx](https://i18n-img.github.io/en-US/1.avif)"
//   );
//
//   // 测试引号不匹配的情况
//   let md = r#"<img src="https://i18n-img.github.io/en/1.avif>"#; // 缺少结束引号
//   let mdli = mdli::md_parse(md);
//   assert_eq!(
//     var_url.replace(&mdli, "zh"),
//     r#"<img src="https://i18n-img.github.io/en/1.avif>"#
//   );
//
//   Ok(())
// }
//
// #[test]
// fn test_complex_markdown() -> Result<()> {
//   let patterns = vec!["https://i18n-img.github.io/", "https://example.com/", "/"];
//   let var_url = VarUrl::new(patterns, "en")?;
//
//   // 测试复杂的 Markdown 结构
//   let md = r#"# Main Title
//
// ## Section 1
// Here's a paragraph with an ![inline image](https://i18n-img.github.io/en/inline.png) and a [link](https://i18n-img.github.io/en/doc.md) mixed in.
//
// ### Subsection with code
//
// ```rust
// let url = "https://i18n-img.github.io/en/test.png"; // 这个不应该被替换
// ```
//
//
// ## Complex HTML
// <div class="container">
//   <img src="https://i18n-img.github.io/en/1.jpg" alt="test1"/>
//   <video src="https://i18n-img.github.io/en/video.mp4" controls>
//     <source src="https://i18n-img.github.io/en/fallback.webm" type="video/webm">
//   </video>
//   <a href="https://i18n-img.github.io/en/page.html" style="color: red">
//     <img src="/en/icon.svg" alt="icon"/>
//   </a>
// </div>
//
// ## Mixed Content
// 1. First item with ![img](/en/list1.png)
// 2. Second item with <img src="https://i18n-img.github.io/en/list2.jpg"/>
// 3. Third item with [link](//example.com/en/no-replace.html)
//
// > Blockquote with ![nested image](https://i18n-img.github.io/en/quote.png)
// > And [nested link](/en/quote.md)
// > And <a href="https://i18n-img.github.io/en/quote.html">HTML link</a>
//
// | Column 1 | Column 2 |
// |----------|----------|
// | ![t1](/en/table1.png) | [t2](https://i18n-img.github.io/en/table2.md) |
// | <img src="/en/table3.jpg"/> | <a href="https://i18n-img.github.io/en/table4.html">link</a> |
//
// * List with ![nested](https://i18n-img.github.io/en/list1.png)
//   * Sublist with [link](/en/list2.md)
//     * Deep nest <img src="https://i18n-img.github.io/en/list3.jpg">
//
// <details>
// <summary>Expandable section with ![image](https://i18n-img.github.io/en/details.png)</summary>
//
// * Hidden ![content](/en/hidden1.jpg)
// * More <img src="https://i18n-img.github.io/en/hidden2.png"/>
// </details>"#;
//
//   let mdli = mdli::md_parse(md);
//   let result = var_url.replace(&mdli, "zh");
//
//   // 验证替换结果
//   assert!(result.contains("![inline image](https://i18n-img.github.io/zh/inline.png)"));
//   assert!(result.contains("[link](https://i18n-img.github.io/zh/doc.md)"));
//   assert!(result.contains("let url = \"https://i18n-img.github.io/en/test.png\"")); // 代码块中不替换
//   assert!(result.contains(r#"<img src="https://i18n-img.github.io/zh/1.jpg""#));
//   assert!(result.contains(r#"<video src="https://i18n-img.github.io/zh/video.mp4""#));
//   assert!(result.contains(r#"<source src="https://i18n-img.github.io/zh/fallback.webm""#));
//   assert!(result.contains(r#"<a href="https://i18n-img.github.io/zh/page.html""#));
//   assert!(result.contains(r#"<img src="/zh/icon.svg""#));
//   assert!(result.contains("![img](/zh/list1.png)"));
//   assert!(result.contains(r#"<img src="https://i18n-img.github.io/zh/list2.jpg"/>"#));
//   assert!(result.contains("[link](//example.com/en/no-replace.html)")); // 不替换 //
//   assert!(result.contains("![nested image](https://i18n-img.github.io/zh/quote.png)"));
//   assert!(result.contains("[nested link](/zh/quote.md)"));
//   assert!(result.contains(r#"<a href="https://i18n-img.github.io/zh/quote.html">"#));
//   assert!(result.contains("![t1](/zh/table1.png)"));
//   assert!(result.contains("[t2](https://i18n-img.github.io/zh/table2.md)"));
//   assert!(result.contains(r#"<img src="/zh/table3.jpg"/>"#));
//   assert!(result.contains(r#"<a href="https://i18n-img.github.io/zh/table4.html">"#));
//
//   Ok(())
// }
//
// #[test]
// fn test_ugly_cases() -> Result<()> {
//   let patterns = vec!["https://i18n-img.github.io/", "/"];
//   let var_url = VarUrl::new(patterns, "en")?;
//
//   // 测试不完整或畸形的 HTML/Markdown
//   let md = r#"
// ![broken](https://i18n-img.github.io/en/
// ![no-close](https://i18n-img.github.io/en/test.png
// [broken](/en/
// <img src="https://i18n-img.github.io/en/test.png
// <img src=https://i18n-img.github.io/en/test.png>
// <a href="https://i18n-img.github.io/en/test.html>broken link</a>
// <a href=https://i18n-img.github.io/en/test.html>no quotes</a>
// ](https://i18n-img.github.io/en/weird.png)
// src="https://i18n-img.github.io/en/test.png"
// href="https://i18n-img.github.io/en/test.html"
// ![img](src="https://i18n-img.github.io/en/test.png")
// [link](href="https://i18n-img.github.io/en/test.html")
// "#;
//   let mdli = mdli::md_parse(md);
//   let result = var_url.replace(&mdli, "zh");
//   // 确保原始内容没有被错误替换
//   assert_eq!(result, md);
//
//   // 测试嵌套的 URL
//   let md = r#"
// [![nested](https://i18n-img.github.io/en/inner.png)](https://i18n-img.github.io/en/outer.png)
// <a href="https://i18n-img.github.io/en/outer.html"><img src="https://i18n-img.github.io/en/inner.jpg"></a>
// [[![double](https://i18n-img.github.io/en/double.png)]](https://i18n-img.github.io/en/weird.html)
// "#;
//   let mdli = mdli::md_parse(md);
//   let result = var_url.replace(&mdli, "zh");
//   assert!(result.contains("https://i18n-img.github.io/zh/inner.png"));
//   assert!(result.contains("https://i18n-img.github.io/zh/outer.png"));
//   assert!(result.contains("https://i18n-img.github.io/zh/inner.jpg"));
//   assert!(result.contains("https://i18n-img.github.io/zh/outer.html"));
//
//   // 测试特殊字符
//   let md = r#"
// ![special&<>"'](https://i18n-img.github.io/en/special&<>"'.png)
// <img src="https://i18n-img.github.io/en/special&<>'\".jpg" alt="test"/>
// <a href="https://i18n-img.github.io/en/special&<>'\".html">link</a>
// [link](/en/special&<>"'.md)
// "#;
//   let mdli = mdli::md_parse(md);
//   let result = var_url.replace(&mdli, "zh");
//   assert!(result.contains("https://i18n-img.github.io/zh/special&<>\"'.png"));
//   assert!(result.contains("https://i18n-img.github.io/zh/special&<>'\\\".jpg"));
//   assert!(result.contains("https://i18n-img.github.io/zh/special&<>'\\\".html"));
//   assert!(result.contains("/zh/special&<>\"'.md"));
//
//   Ok(())
// }
