# mdli

```rust
use mdli::{md_parse, Md};

fn print_test_result(name: &str, success: bool) {
  if success {
    println!("✅ {}", name);
  } else {
    println!("❌ {}", name);
  }
}

fn join_md_results(results: &Vec<Md>) -> String {
  results.iter().map(|md| md.str).collect::<String>()
}

fn run_test(name: &str, input: &str) {
  let result = md_parse(input);
  dbg!(&result);
  let joined = join_md_results(&result);

  let success = joined == input;
  print_test_result(name, success);
  assert_eq!(input, joined, "\n期望: {}\n实际: {}", input, joined);
}

#[test]
fn test_md_parse() {
  let test_cases = vec![
      ("普通文本测试",
       "这是一段普通的中文文字。"),

      ("代码块测试",
      "前面的文字\n```\n这是代码块\n包含多行\n```\n后面的文字"),

      ("行内代码测试",
       "这是`行内代码`测试，`另一个行内代码`结束"),

      ("混合情况测试",
       "开始文字`行内代码`中间文字\n```\n代码块内容\n多行内容\n```\n结束文字"),

      ("未闭合代码块测试",
       "开始文字\n```\n未闭合的代码块\n继续写内容"),

      ("未闭合行内代码测试",
       "开始文字`未闭合的行内代码继续写"),

      ("空文本测试",
       ""),

      ("空白字符测试",
       "    \n    \n    "),

      ("连续代码块测试",
       "```\n第一个代码块\n```\n```\n第二个代码块\n```"),

      ("中文特殊字符测试",
       "测试特殊字符：`你好！@#￥%……&*（）——+`结束"),

      ("复杂混合测试",
       "第一段`行内代码1`文字\n```\n代码块1\n包含`行内代码`\n```\n第二段`行内代码2`文字\n```\n代码块2\n```\n最后的文字"),

      ("空代码块测试",
       "前面\n```\n```\n后面"),

      ("空行内代码测试",
       "前面``后面"),

      ("连续行内代码测试",
       "文字`代码1``代码2``代码3`文字"),

      ("包含换行测试",
       "第一行\r\n第二行\n`行内代码`\n第三行"),

      ("中文标点符号测试",
       "测试句号。测试逗号，测试冒号：`行内代码`测试引号\"测试\""),

      ("多重嵌套测试",
       "外层文字\n```\n`内层代码`块内容\n```\n继续"),

       ("结尾没有回车的测试",
       "外层文字\n```\n`内层代码`块内容\n```"),
  ];

  for (name, input) in test_cases {
    run_test(name, input);
  }
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