# bing : a search engine

1. Tantivy 如何为长篇文章截取200字的高亮摘要？（中文重述）
在 Rust 的全文检索引擎库 Tantivy 中，要从一篇长文章里，根据用户的搜索词，提取出一段大约200字左右的摘要，并高亮其中的关键词，主要通过 SnippetGenerator 来实现。这对于提升搜索结果的用户体验至关重要。

核心原理
Tantivy 的摘要生成并非简单地在关键词前后截取固定长度的文本。它采用了一种更智能的评分机制，以找出最能体现搜索结果与查询相关性的文本片段。

依赖位置信息：首先，你的索引字段必须包含词元的位置信息。这需要在定义 Schema 时，将字段的 IndexRecordOption 设置为 WithFreqsAndPositions。同时，为了能从索引中取回原文来生成摘要，字段必须设置为 STORED。

片段评分 (Fragment Scoring)：SnippetGenerator 会在内部将整篇文档分割成多个小的“片段”（fragments）。然后，它会根据以下几点为每个片段打分：

关键词密度：片段中包含的搜索关键词越多，分数越高。

关键词集中度：如果多个搜索关键词在片段中紧密相邻地出现，这个片段的相关性就更高，得分也更高。

长度预算：SnippetGenerator 会尽量选择长度接近你设定的最大字符数（例如200）的片段。

最佳片段选择：综合评分后，SnippetGenerator 会选出得分最高的那个片段作为最终的摘要（Snippet）。

HTML高亮：选出最佳片段后，其内部的 Highlighter 会将与查询匹配的关键词用指定的 HTML 标签（默认为 <b>）包裹起来，最终通过调用 .to_html() 方法生成高亮后的 HTML 字符串。

代码演示
下面的 Rust 代码完整演示了如何建立索引、添加长文档、进行搜索，并最终生成一个大约200字符的高亮摘要。

Rust

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, ReloadPolicy};
use tantivy::snippet::SnippetGenerator;
use tempfile::TempDir;

fn main() -> tantivy::Result<()> {
    // 1. 定义 Schema
    let mut schema_builder = Schema::builder();
    // 确保字段配置正确：启用存储(storing)和位置信息(positions)
    let text_options = TextOptions::default()
        .set_storing_enabled(true)
        .set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("en_stem")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        );
    let title = schema_builder.add_text_field("title", text_options.clone());
    let body = schema_builder.add_text_field("body", text_options);
    let schema = schema_builder.build();

    // 2. 创建索引
    let index_path = TempDir::new()?;
    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let mut index_writer = index.writer(50_000_000)?;

    // 3. 添加一篇长文档
    let long_text = "Tantivy is a full-text search engine library written in Rust. \
    It is highly inspired by Apache Lucene. It is designed to be a modern, \
    performant, and easy-to-use library for building search applications. \
    One of the key features of a search engine is the ability to display a snippet \
    of the document with the search terms highlighted. This is crucial for users \
    to quickly understand the context of the search results. Tantivy provides a \
    powerful SnippetGenerator for this purpose. This generator can create a concise \
    and relevant fragment of the original text. The length of this fragment, or snippet, \
    can be controlled. For instance, we can aim for a snippet of around 200 characters. \
    The highlighter will then wrap the matched terms with HTML tags, like <b>, to make them stand out. \
    This combination of snippet generation and highlighting is essential for a good user experience in any search interface.";

    index_writer.add_document(doc!(
        title => "Tantivy Highlighting Example",
        body => long_text
    ))?;
    index_writer.commit()?;

    // 4. 准备搜索
    let reader = index.reader_builder().reload_policy(ReloadPolicy::OnCommit).try_into()?;
    let searcher = reader.searcher();
    // 注意：QueryParser 需要知道哪些字段是可搜索的
    let query_parser = QueryParser::for_index(&index, vec![title, body]);

    // 5. 执行搜索
    let query = query_parser.parse_query("search library")?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    // 6. 为 'body' 字段创建 SnippetGenerator 并生成高亮摘要
    // `SnippetGenerator` 需要知道你想从哪个字段生成摘要
    let mut snippet_generator = SnippetGenerator::from_query(&query, &searcher, body);

    // **核心：设置摘要的最大字符数**
    snippet_generator.set_max_num_chars(200);

    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("文档得分: {}", score);

        let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);

        // 生成高亮后的 HTML
        let highlighted_html = snippet.to_html();

        println!("高亮摘要 (约200字符):");
        println!("{}", highlighted_html);
        println!("---");
    }

    Ok(())
}
2. 如果我有多个字段（例如标题和正文），应该怎么处理？
当你的搜索查询可能匹配多个字段（如 title 和 body）时，你需要决定如何展示高亮摘要。通常有两种策略：

为每个匹配的字段都生成摘要：分别对标题和正文生成摘要，然后都展示给用户。

选择最佳摘要：对每个可能匹配的字段都生成摘要，然后通过某种逻辑（例如，哪个摘要包含的关键词更多，或者哪个字段更重要）选择一个最佳的摘要展示。

SnippetGenerator 本身在创建时是针对单个字段的。SnippetGenerator::from_query(&query, &searcher, field) 的第三个参数 field 明确了它将从哪个字段中提取原文并生成摘要。

因此，处理多字段高亮的正确做法是：为每个需要高亮的字段分别创建一个 SnippetGenerator。

代码演示 (处理多字段)
以下代码展示了如何为 title 和 body 两个字段分别生成高亮摘要。

Rust

// ... (接续上一个例子的 main 函数，从步骤 6 开始修改)

// 6. 为多个字段创建 SnippetGenerator 并生成高亮摘要
let fields_to_highlight = vec![title, body]; // 定义你想要高亮的字段列表

for (score, doc_address) in top_docs {
    let retrieved_doc = searcher.doc(doc_address)?;
    println!("文档得分: {}", score);
    println!("文档: {}", schema.to_json(&retrieved_doc));
    println!("---");

    for field in &fields_to_highlight {
        // 为当前循环的字段创建一个 SnippetGenerator
        let mut snippet_generator = SnippetGenerator::from_query(&query, &searcher, *field);
        snippet_generator.set_max_num_chars(200); // 同样可以设置长度限制

        let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);

        // 检查生成的 snippet 是否为空（即该字段中没有匹配的词）
        if !snippet.is_empty() {
             // 获取字段名称，以便更好地展示
            let field_name = schema.get_field_name(*field);
            println!("字段 '{}' 的高亮摘要:", field_name);
            println!("{}", snippet.to_html());
            println!("---");
        }
    }
}
多字段处理逻辑解析
定义目标字段：我们创建了一个 Vec<Field>，包含了所有我们希望尝试生成高亮的字段 (title, body)。

遍历字段：在获取到每个搜索结果文档 (retrieved_doc) 后，我们遍历这个字段列表。

独立创建 SnippetGenerator：在循环内部，我们为每一个字段都创建了一个新的 SnippetGenerator 实例。这是因为每个生成器都需要知道它具体要处理哪个字段的原文。

生成并检查摘要：调用 snippet_from_doc 生成摘要。由于搜索词可能只存在于部分字段中，所以我们用 snippet.is_empty() 来判断当前字段是否真的匹配到了关键词并成功生成了摘要。

分别展示：如果摘要不为空，我们就将其高亮后的 HTML 打印出来，并附上字段名，让用户清楚地知道这个摘要来自标题还是正文。

这种方法提供了最大的灵活性，你可以根据你的业务需求，决定是将所有字段的摘要都展示出来，还是根据摘要的质量（例如，摘要的评分 snippet.score()）只选择最好的一个。

```rust
use std::time::Duration;

use anyhow::Result;
use bing::Doc;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[test]
fn test_search_engine() -> Result<()> {
  let path = "/tmp/bing";

  let db = bing::doc::open(path)?;

  let mut writer = db.writer()?;

  let id = 99;

  let doc1 = Doc {
    id,
    uid: 100,
    org_id: 2,
    repo_id: 10,
    tag_li: vec!["abc".into(), "电动车品牌".into(), "xyz".into()],
    ts: 1640995200, // 2022-01-01
    title: "YES Good 搜索引擎".into(),
    txt: String::from_utf8_lossy(include_bytes!("./blog.md")).into(),
  };

  let doc_id = writer.add(doc1)?;
  dbg!(doc_id);

  let mut seacher = db.searcher()?;
  let li = seacher.search(bing::search::Query {
    query: "品牌".into(),
    uid: 0,
    org_id: 0,
    repo_id_li: vec![],
    tag_li: vec![],
    ts_begin: None,
    ts_end: None,
    limit: 10,
    offset: 0,
    snippet_max_num_chars: 300,
  })?;
  dbg!(li);

  writer.rm(id)?;

  std::thread::sleep(Duration::from_secs(1));
  let li = seacher.search(bing::search::Query {
    query: "品牌".into(),
    ..Default::default()
  })?;
  dbg!(li);

  Ok(())
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
