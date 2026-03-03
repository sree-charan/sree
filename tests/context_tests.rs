use sree::context::system::SystemContext;

#[test]
fn test_system_context_creation() {
    let ctx = SystemContext::new();
    
    assert!(!ctx.os.is_empty());
    assert!(!ctx.arch.is_empty());
    assert!(!ctx.cwd.is_empty());
    assert!(!ctx.time.is_empty());
}

#[test]
fn test_system_prompt_generation() {
    let ctx = SystemContext::new();
    let prompt = ctx.generate_system_prompt();
    
    assert!(prompt.contains("sree"));
    assert!(prompt.contains(&ctx.os));
    assert!(prompt.contains(&ctx.cwd));
    assert!(prompt.contains("tools"));
    assert!(prompt.contains("Safety Rules"));
}
