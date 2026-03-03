use sree::ui::widgets::file_tree::FileTreeNode;
use sree::ui::widgets::diff_view::{DiffView, DiffLine};
use sree::context::files::FileContext;

#[test]
fn test_file_tree_node_creation() {
    let node = FileTreeNode::new("test.rs".to_string(), false, 0);
    assert_eq!(node.name, "test.rs");
    assert!(!node.is_dir);
    assert_eq!(node.depth, 0);
    assert_eq!(node.children.len(), 0);
}

#[test]
fn test_file_tree_rendering() {
    let mut root = FileTreeNode::new("src".to_string(), true, 0);
    root.children.push(FileTreeNode::new("main.rs".to_string(), false, 1));
    root.children.push(FileTreeNode::new("lib.rs".to_string(), false, 1));
    
    let lines = root.render();
    assert_eq!(lines.len(), 3); // root + 2 children
}

#[test]
fn test_diff_view_creation() {
    let lines = vec![
        DiffLine::Added("new line".to_string()),
        DiffLine::Removed("old line".to_string()),
        DiffLine::Context("unchanged".to_string()),
    ];
    
    let diff = DiffView::new(lines);
    assert_eq!(diff.lines.len(), 3);
}

#[test]
fn test_diff_view_from_strings() {
    let old = "line 1\nline 2\nline 3";
    let new = "line 1\nmodified line 2\nline 3";
    
    let diff = DiffView::from_strings(old, new);
    let rendered = diff.render();
    
    // Should have context, removed, added, context
    assert!(rendered.len() >= 4);
}

#[test]
fn test_diff_view_rendering() {
    let lines = vec![
        DiffLine::Added("added".to_string()),
        DiffLine::Removed("removed".to_string()),
        DiffLine::Context("context".to_string()),
    ];
    
    let diff = DiffView::new(lines);
    let rendered = diff.render();
    
    assert_eq!(rendered.len(), 3);
}

#[test]
fn test_file_context_creation() {
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "line 1").unwrap();
    writeln!(temp_file, "line 2").unwrap();
    writeln!(temp_file, "line 3").unwrap();
    
    let context = FileContext::from_path(temp_file.path()).unwrap();
    assert_eq!(context.line_count, 3);
    assert!(context.content.contains("line 1"));
}

#[test]
fn test_file_context_formatting() {
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "test content").unwrap();
    
    let context = FileContext::from_path(temp_file.path()).unwrap();
    let formatted = context.format_for_context();
    
    assert!(formatted.contains("File:"));
    assert!(formatted.contains("Lines:"));
    assert!(formatted.contains("test content"));
}
