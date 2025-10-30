use md2htm_without_p::to_htm;

#[test]
fn test_basic_text() {
  let md = "Hello, world!";
  let html = to_htm(md);
  assert_eq!(html, "Hello, world!");
}

#[test]
fn test_formatting() {
  let md = "**Bold** and *italic* text";
  let html = to_htm(md);
  assert_eq!(html, "<strong>Bold</strong> and <em>italic</em> text");
}

#[test]
fn test_links() {
  let md = "[Link](https://example.com)";
  let html = to_htm(md);
  assert_eq!(html, "<a href=\"https://example.com\">Link</a>");
}

#[test]
fn test_headings() {
  let md = "# Heading 1\n## Heading 2";
  let html = to_htm(md);
  assert_eq!(html, "<h1>Heading 1</h1>\n<h2>Heading 2</h2>");
}

#[test]
fn test_no_paragraph_tags() {
  let md = "Paragraph 1\n\nParagraph 2";
  let html = to_htm(md);
  assert_eq!(html, "Paragraph 1\nParagraph 2");
}

#[test]
fn test_mixed_content() {
  let md = "# Title\nSome text with **bold**.\nAnother paragraph.";
  let html = to_htm(md);
  assert_eq!(
    html,
    "<h1>Title</h1>\nSome text with <strong>bold</strong>.\nAnother paragraph."
  );
}

#[test]
fn test_err_htm() {
  let md = "[Website](https://www.open-notebook.ai) - Learn more about the project";
  let html = to_htm(md);
  dbg!(html);
}
