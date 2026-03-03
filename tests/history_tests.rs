use std::fs;

#[test]
fn test_history_file_creation() {
    let temp_dir = std::env::temp_dir().join("sree_test_history");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    
    let history_file = temp_dir.join("history");
    let content = "first command\nsecond command\nthird command";
    fs::write(&history_file, content).unwrap();
    
    let loaded = fs::read_to_string(&history_file).unwrap();
    let lines: Vec<String> = loaded.lines().map(|s| s.to_string()).collect();
    
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "first command");
    assert_eq!(lines[1], "second command");
    assert_eq!(lines[2], "third command");
    
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_history_deduplication() {
    let mut history: Vec<String> = vec![];
    
    let add_to_history = |history: &mut Vec<String>, input: String| {
        if !input.trim().is_empty() && history.last() != Some(&input) {
            history.push(input);
            if history.len() > 100 {
                history.remove(0);
            }
        }
    };
    
    add_to_history(&mut history, "command1".to_string());
    add_to_history(&mut history, "command2".to_string());
    add_to_history(&mut history, "command2".to_string()); // Duplicate
    add_to_history(&mut history, "command3".to_string());
    
    assert_eq!(history.len(), 3);
    assert_eq!(history[0], "command1");
    assert_eq!(history[1], "command2");
    assert_eq!(history[2], "command3");
}

#[test]
fn test_history_max_size() {
    let mut history: Vec<String> = vec![];
    
    let add_to_history = |history: &mut Vec<String>, input: String| {
        if !input.trim().is_empty() && history.last() != Some(&input) {
            history.push(input);
            if history.len() > 100 {
                history.remove(0);
            }
        }
    };
    
    // Add 105 items
    for i in 0..105 {
        add_to_history(&mut history, format!("command{}", i));
    }
    
    assert_eq!(history.len(), 100);
    assert_eq!(history[0], "command5"); // First 5 should be removed
    assert_eq!(history[99], "command104");
}

#[test]
fn test_history_navigation_logic() {
    let history = vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ];
    
    // Start with no index
    let mut history_index: Option<usize> = None;
    
    // Navigate up (should go to last item)
    if history_index.is_none() {
        history_index = Some(history.len() - 1);
    }
    assert_eq!(history_index, Some(2));
    assert_eq!(history[history_index.unwrap()], "third");
    
    // Navigate up again
    if let Some(idx) = history_index {
        if idx > 0 {
            history_index = Some(idx - 1);
        }
    }
    assert_eq!(history_index, Some(1));
    assert_eq!(history[history_index.unwrap()], "second");
    
    // Navigate up again
    if let Some(idx) = history_index {
        if idx > 0 {
            history_index = Some(idx - 1);
        }
    }
    assert_eq!(history_index, Some(0));
    assert_eq!(history[history_index.unwrap()], "first");
    
    // Navigate down
    if let Some(idx) = history_index {
        if idx < history.len() - 1 {
            history_index = Some(idx + 1);
        }
    }
    assert_eq!(history_index, Some(1));
    assert_eq!(history[history_index.unwrap()], "second");
    
    // Navigate down to end
    if let Some(idx) = history_index {
        if idx < history.len() - 1 {
            history_index = Some(idx + 1);
        }
    }
    assert_eq!(history_index, Some(2));
    
    // Navigate down past end (should clear index)
    if let Some(idx) = history_index {
        if idx >= history.len() - 1 {
            history_index = None;
        }
    }
    assert_eq!(history_index, None);
}

#[test]
fn test_empty_history_navigation() {
    let history: Vec<String> = vec![];
    let mut history_index: Option<usize> = None;
    
    // Try to navigate up with empty history
    if !history.is_empty() {
        if history_index.is_none() {
            history_index = Some(history.len() - 1);
        }
    }
    
    assert_eq!(history_index, None);
}
