use sree::ui::markdown::render_markdown;

#[test]
fn test_plain_text() {
    let text = "Hello, world!";
    let lines = render_markdown(text);
    assert!(!lines.is_empty());
}

#[test]
fn test_code_block() {
    let text = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let lines = render_markdown(text);
    assert!(lines.len() > 4); // Header + code lines + footer
}

#[test]
fn test_inline_code() {
    let text = "Use `cargo build` to compile.";
    let lines = render_markdown(text);
    assert!(!lines.is_empty());
}

#[test]
fn test_headers() {
    let text = "# Header 1\n## Header 2";
    let lines = render_markdown(text);
    assert!(lines.len() >= 2);
}

#[test]
fn test_bold_and_italic() {
    let text = "This is **bold** and *italic* text.";
    let lines = render_markdown(text);
    assert!(!lines.is_empty());
}

#[test]
fn test_mixed_content() {
    let text = "# Title\n\nSome text with `code` and **bold**.\n\n```python\nprint('hello')\n```";
    let lines = render_markdown(text);
    assert!(lines.len() > 5);
}

#[test]
fn test_unordered_list() {
    let text = "- Item 1\n- Item 2\n- Item 3";
    let lines = render_markdown(text);
    assert!(lines.len() >= 3);
}

#[test]
fn test_ordered_list() {
    let text = "1. First\n2. Second\n3. Third";
    let lines = render_markdown(text);
    assert!(lines.len() >= 3);
}

#[test]
fn test_nested_list() {
    let text = "- Item 1\n  - Nested 1\n  - Nested 2\n- Item 2";
    let lines = render_markdown(text);
    assert!(lines.len() >= 4);
}

#[test]
fn test_blockquote() {
    let text = "> This is a quote\n> Second line";
    let lines = render_markdown(text);
    assert!(lines.len() >= 2);
}

#[test]
fn test_table() {
    let markdown = "| Name | Age |\n|------|-----|\n| Alice | 30 |\n| Bob | 25 |";
    let lines = render_markdown(markdown);
    let result_text = lines.iter()
        .map(|l| l.spans.iter().map(|s| s.content.as_ref()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    
    assert!(result_text.contains("Name"));
    assert!(result_text.contains("Age"));
    assert!(result_text.contains("Alice"));
    assert!(result_text.contains("Bob"));
}
