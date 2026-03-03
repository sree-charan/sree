use sree::ui::widgets::spinner::Spinner;

#[test]
fn test_spinner_animation() {
    let mut spinner = Spinner::new();
    
    // Test initial state
    let line = spinner.render();
    assert!(!line.spans.is_empty());
    
    // Test ticking advances frame
    let initial = spinner.render();
    spinner.tick();
    let after_tick = spinner.render();
    
    // The spinner should show different frames
    assert_ne!(initial.spans[0].content, after_tick.spans[0].content);
}

#[test]
fn test_spinner_cycles() {
    let mut spinner = Spinner::new();
    
    // Tick through all frames and verify it cycles
    for _ in 0..20 {
        spinner.tick();
        let line = spinner.render();
        assert!(!line.spans.is_empty());
    }
}
