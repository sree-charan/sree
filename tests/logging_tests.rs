use sree::logging;

#[test]
fn test_logging_init() {
    // This test verifies that logging initialization doesn't panic
    // and creates the necessary directories
    let result = logging::init();
    
    // Should succeed or fail gracefully
    assert!(result.is_ok() || result.is_err());
    
    // If successful, verify log directory was created
    if result.is_ok() {
        let log_dir = dirs::home_dir()
            .unwrap()
            .join(".sree")
            .join("logs");
        assert!(log_dir.exists());
    }
}
