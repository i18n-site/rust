use pulldown_cmark::{Event, Parser, Tag, TagEnd, html};

pub fn to_htm(md: impl AsRef<str>) -> String {
  let md = md.as_ref();

  // 解析 Markdown
  let parser = Parser::new(md);

  // 过滤掉 p 标签
  let parser = parser.filter_map(|event| match event {
    // 跳过段落开始和结束标签
    Event::Start(Tag::Paragraph) => None,
    Event::End(TagEnd::Paragraph) => Some(Event::InlineHtml('\n'.into())),
    // 保留其他所有事件
    _ => Some(event),
  });

  // 将事件转换为 HTML
  let mut htm = String::new();
  html::push_html(&mut htm, parser);

  htm.trim_end().into()
}
