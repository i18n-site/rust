use std::{
  fs,
  path::{Path, PathBuf},
};

use mdli::{fmt, parse};

fn test_fp(dir: &Path, path: impl AsRef<Path>) -> aok::Void {
  let path = path.as_ref();
  let txt = fs::read_to_string(path)?;
  let txt = fmt(&txt);
  let rel = path.strip_prefix(dir).unwrap().to_string_lossy();
  // println!("文件: {:?}\n内容:\n{}\n", rel, txt);
  println!("\n→ {}", rel);
  let result = parse(&txt);
  for i in &result.li {
    println!("{:?} {:?}", i.kind, i.str);
  }
  assert_eq!(txt, result.join(), "{rel} not same");
  aok::OK
}

#[test]
fn test_md() -> aok::Void {
  let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  dir.push("tests/md");

  if let Ok(test_file) = std::env::var("TEST_FILE") {
    test_fp(&dir, dir.join(test_file))?;
  } else {
    for entry in fs::read_dir(&dir)? {
      let entry = entry?;
      let path = entry.path();

      if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
        test_fp(&dir, &path)?;
      }
    }
  }

  aok::OK
}

// fn run_test(name: &str, input: &str) {
//   let input = fmt(input);
//   let result = parse(&input);
//   // println!("# {}", name,);
//   // dbg!(&result);
//   // for (pos, i) in result.txt_iter() {
//   //   println!("{pos} {i}");
//   // }
//   // println!("");
//   let joined = result.join();
//   assert_eq!(
//     input, joined,
//     "\n{}\n期望: {}\n实际: {}",
//     name, input, joined
//   );
// }
//
// #[test]
// fn test_parse() {
//   let test_cases = vec![
//         ("混合情况测试", "开始文字`行内代码`中间文字\n```\n代码块内容\n多行内容\n```\n结束文字"),
//         ("行内代码测试",
//           "这是`行内代码`测试，`另一个行内代码`结束"),
//
//         (
//
//           "简单代码",
//           "\\`not code`\n\\\\`code`\nxx ``` ` ``` `code` \n```rust\n println!(1)\n```"
//           ),
//           (
//     "单行代码转义", r#"1`\``2"#
//
//             ),
//           (
//     "多行代码转义", "```\n1\\```\n2```\n"
//
//             ),
//           (
//             "HTML注释",
//             "测试<!--注释-->\n   \n<!-- 这是\n多行注释\n-->没有内容<!---->\n\n   \n\n",
//             ),
//           (
//             "单个HTML",
//             "<h2>标题</h2>\n<a title=\"中文\" href=\"https://x.x?x&1\">测试</a>",
//             ),
//           (
//             "简单HTML",
//             "HTML文本\n<a \nhref=\"https://x.x?x&1\"\n>\n闭合</a>测<b\n>试</b>一下",
//             ),
//           ("普通文本测试",
//             "这是一段普通的中文文字。"),
//
//           ("代码块测试",
//           "前面的文字\n```\n这是代码块\n包含多行\n```\n后面的文字"),
//
//
//
//           ("未闭合代码块测试",
//             "开始文字\n```\n未闭合的代码块\n继续写内容"),
//
//           ("未闭合行内代码测试",
//             "开始文字`未闭合的行内代码继续写"),
//
//           ("空文本测试",
//             ""),
//
//           ("空白字符测试",
//             "    \n    \n    "),
//
//           ("连续代码块测试",
//             "```\n第一个代码块\n```\n```\n第二个代码块\n```"),
//
//           ("中文特殊字符测试",
//             "测试特殊字符：`你好！@#￥%……&*（）——+`结束"),
//
//           ("复杂混合测试",
//             "第一段`行内代码1`文字\n```\n代码块1\n包含`行内代码`\n```\n第二段`行内代码2`文字\n```\n代码块2\n```\n最后的文字"),
//
//           ("空代码块测试",
//             "前面\n```\n```\n后面"),
//
//           ("空行内代码测试",
//             "前面``后面"),
//
//           ("连续行内代码测试",
//             "文字`代码1``代码2``代码3`文字"),
//
//           ("包含换行测试",
//             "第一行\r\n第二行\n`行内代码`\n第三行"),
//
//           ("中文标点符号测试",
//             "测试句号。测试逗号，测试冒号：`行内代码`测试引号\"测试\""),
//
//           ("多重嵌套测试",
//             "外层文字\n```\n`内层代码`块内容\n```\n继续"),
//
//             ("结尾没有回车的测试",
//             "\r\n    \r\r外层文字\n```\n`内层代码`块内容\n```"),
//           (
//             "图片&链接",
//             "测试`code1`文字0`code2`文字1![图片描述](img_url链接)文字2\n文字3[链接文本](link链接)"
//           ),
//         ("空",""),
//         ("空行","\n\n"),
//         ("有缩进的行","  1 \n    2  \n\n 3  "),
//         ("标题","# 测试\n## 2   \n  ## 3\n  #4"),
//         ("行内代码内嵌代码","行内内嵌` ```i18n `测试"),
//         ("行内代码转义-1","行内内嵌` \\`i18n `测试"),
//         ("行内代码转义-2","`\\"),
//         ("行内代码转义-3","`\\`"),
//         ("行内代码转义-4", "`\\``"),
//     ("行内代码行首空白", "   测试`\\``\n  测试`2`测试"),
//   ];
//
//   // let mut limit = 2;
//   for (name, input) in test_cases {
//     // if limit == 0 {
//     //   break;
//     // }
//     // limit -= 1;
//     run_test(name, input);
//     // break;
//   }
// }
