use mdli::md_parse;

fn print_test_result(name: &str, success: bool) {
  if success {
    println!("✅ {}", name);
  } else {
    println!("❌ {}", name);
  }
}

fn run_test(name: &str, input: &str) {
  let result = md_parse(input);
  let joined = result.join();
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
       "\r\n    \r\r外层文字\n```\n`内层代码`块内容\n```"),
  ];

  for (name, input) in test_cases {
    run_test(name, input);
  }
}
