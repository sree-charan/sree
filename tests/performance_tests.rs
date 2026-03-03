use sree::context::conversation::Conversation;
use sree::llm::messages::{ApiMessage, ContentBlock};
use std::time::Instant;

#[test]
fn test_conversation_performance_small() {
    // Test performance with a small conversation (10 messages)
    let mut conv = Conversation::new(200_000);
    
    let start = Instant::now();
    for i in 0..10 {
        conv.add_message(ApiMessage {
            role: "user".to_string(),
            content: vec![ContentBlock::Text {
                text: format!("Message {}", i),
            }],
        });
    }
    let duration = start.elapsed();
    
    assert_eq!(conv.messages().len(), 10);
    assert!(duration.as_millis() < 10, "Small conversation should be fast");
}

#[test]
fn test_conversation_performance_medium() {
    // Test performance with a medium conversation (100 messages)
    let mut conv = Conversation::new(200_000);
    
    let start = Instant::now();
    for i in 0..100 {
        conv.add_message(ApiMessage {
            role: "user".to_string(),
            content: vec![ContentBlock::Text {
                text: format!("Message {}", i),
            }],
        });
    }
    let duration = start.elapsed();
    
    assert_eq!(conv.messages().len(), 100);
    assert!(duration.as_millis() < 50, "Medium conversation should be reasonably fast");
}

#[test]
fn test_conversation_performance_large() {
    // Test performance with a large conversation (1000 messages)
    let mut conv = Conversation::new(200_000);
    
    let start = Instant::now();
    for i in 0..1000 {
        conv.add_message(ApiMessage {
            role: "user".to_string(),
            content: vec![ContentBlock::Text {
                text: format!("Message {}", i),
            }],
        });
    }
    let duration = start.elapsed();
    
    assert_eq!(conv.messages().len(), 1000);
    assert!(duration.as_millis() < 500, "Large conversation should complete in reasonable time");
}

#[test]
fn test_token_counting_performance() {
    // Test token counting performance with various message sizes
    let mut conv = Conversation::new(200_000);
    
    // Add messages with varying content sizes
    let sizes = vec![10, 100, 1000, 10000];
    
    for size in sizes {
        let text = "a".repeat(size);
        let start = Instant::now();
        
        conv.add_message(ApiMessage {
            role: "user".to_string(),
            content: vec![ContentBlock::Text { text }],
        });
        
        let duration = start.elapsed();
        assert!(duration.as_micros() < 1000, "Token counting should be fast");
    }
}

#[test]
fn test_message_cloning_performance() {
    // Test that message cloning is efficient
    let message = ApiMessage {
        role: "user".to_string(),
        content: vec![
            ContentBlock::Text {
                text: "a".repeat(10000),
            },
        ],
    };
    
    let start = Instant::now();
    for _ in 0..100 {
        let _cloned = message.clone();
    }
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 50, "Message cloning should be efficient");
}

#[test]
fn test_conversation_truncation_performance() {
    // Test that conversation truncation is efficient
    let mut conv = Conversation::new(1_000); // Very small limit to force truncation
    
    // Add many messages with substantial content to trigger truncation
    for i in 0..50 {
        conv.add_message(ApiMessage {
            role: "user".to_string(),
            content: vec![ContentBlock::Text {
                text: format!("Message {} with substantial content to increase token count. This message contains enough text to contribute meaningfully to the token count and force truncation when we exceed the limit.", i),
            }],
        });
    }
    
    // Truncation should have happened automatically
    assert!(conv.messages().len() < 50, "Conversation should be truncated");
    assert!(conv.estimate_tokens() <= 1_000, "Token count should be under limit");
}
