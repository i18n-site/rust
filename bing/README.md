# bing : a search engine

```rust
pub mod doc;
mod open;
pub use doc::{Doc, DocFiled};
pub use open::open;
pub mod schema;

/*
// --- 更新文档 ---
let doc_id_to_update = "doc_123";

// 1. 创建用于删除的 Term
let id_term = Term::from_field_text(id_field, doc_id_to_update);

// 2. 删除旧文档
index_writer.delete_term(id_term);

// 3. 添加新版本的文档
index_writer.add_document(doc!(
    id_field => doc_id_to_update,
    title_field => "A new title",
    body_field => "The updated body of the document."
))?;

// 4. 提交更改以使更新生效
index_writer.commit()?;


// 4. 准备搜索
let reader = index.reader()?;
let searcher = reader.searcher();
let query_parser = QueryParser::for_index(&index, vec![title, body]);

// 5. 执行查询
// 这个查询将自动使用 BM25 进行评分
let query = query_parser.parse_query("old man")?;
let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

println!("Searching for 'old man':");
for (score, doc_address) in top_docs {
    let retrieved_doc = searcher.doc(doc_address)?;
    // 打印出的 'score' 就是 BM25 计算出的相关性分数
    println!(
        "Score: {:?}, Doc: {:?}",
        score,
        retrieved_doc.get_first(title).unwrap().as_text().unwrap()
    );
}

*/
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
