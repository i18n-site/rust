# mdli

```rust
use mdli::parse;

fn run_test(name: &str, input: &str) {
  println!("# {}", name,);
  let result = parse(input);
  // dbg!(&result);
  for (pos, i) in result.txt_iter() {
    println!("{pos} {i}");
  }
  println!("");
  let joined = result.join();
  assert_eq!(input, joined, "\n期望: {}\n实际: {}", input, joined);
}

#[test]
fn test_parse() {
  let test_cases = vec![
    ("混合情况测试", "开始文字`行内代码`中间文字\n```\n代码块内容\n多行内容\n```\n结束文字"),
    ("行内代码测试",
      "这是`行内代码`测试，`另一个行内代码`结束"),

    (

      "简单代码",
      "\\`not code`\n\\\\`code`\nxx ``` ` ``` `code` \n```rust\n println!(1)\n```"
      ),
      (
"单行代码转义", r#"1`\``2"#

        ),
      (
"多行代码转义", "```\n1\\```\n2```\n"

        ),
      (
        "HTML注释",
        "测试<!--注释-->\n   \n<!-- 这是\n多行注释\n-->没有内容<!---->\n\n   \n\n",
        ),
      (
        "单个HTML",
        "<h2>标题</h2>\n<a title=\"中文\" href=\"https://x.x?x&1\">测试</a>",
        ),
      (
        "简单HTML",
        "HTML文本\n<a \nhref=\"https://x.x?x&1\"\n>\n闭合</a>测<b\n>试</b>一下",
        ),
      ("普通文本测试",
        "这是一段普通的中文文字。"),

      ("代码块测试",
      "前面的文字\n```\n这是代码块\n包含多行\n```\n后面的文字"),



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
        "\r\n    \r\r外层文字\n```\n`内层代码`块内容\n```"),
      (
        "图片&链接",
        "测试`code1`文字0`code2`文字1![图片描述](img_url链接)文字2\n文字3[链接文本](link链接)"
      ),
    ("空",""),
    ("空行","\n\n"),
    ("有缩进的行","  1 \n    2  \n\n 3  "),
    ("标题","# 测试\n## 2   \n  ## 3\n  #4"),
    ("行内代码内嵌代码","行内内嵌` ```i18n `测试"),
    ("行内代码转义-1","行内内嵌` \\`i18n `测试"),
    ("行内代码转义-2","`\\"),
    ("行内代码转义-3","`\\`"),
    ("行内代码转义-4", "`\\``"),
  ];

  // let mut limit = 2;
  for (name, input) in test_cases {
    // if limit == 0 {
    //   break;
    // }
    // limit -= 1;
    run_test(name, input);
    // break;
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