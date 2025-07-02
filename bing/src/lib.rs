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
