use anyhow::{Context, Result};
use aws_sdk_bedrockruntime::{Client as BedrockClient, types::{ContentBlock, Message, ConversationRole, SystemContentBlock, Tool, ToolConfiguration}};
use tokio::sync::mpsc;

use super::messages::ApiRequest;
use super::streaming::StreamEvent;

fn json_to_document(value: serde_json::Value) -> aws_smithy_types::Document {
    match value {
        serde_json::Value::Null => aws_smithy_types::Document::Null,
        serde_json::Value::Bool(b) => aws_smithy_types::Document::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                aws_smithy_types::Document::Number(aws_smithy_types::Number::PosInt(i as u64))
            } else if let Some(f) = n.as_f64() {
                aws_smithy_types::Document::Number(aws_smithy_types::Number::Float(f))
            } else {
                aws_smithy_types::Document::Null
            }
        }
        serde_json::Value::String(s) => aws_smithy_types::Document::String(s),
        serde_json::Value::Array(arr) => {
            aws_smithy_types::Document::Array(arr.into_iter().map(json_to_document).collect())
        }
        serde_json::Value::Object(obj) => {
            aws_smithy_types::Document::Object(
                obj.into_iter()
                    .map(|(k, v)| (k, json_to_document(v)))
                    .collect(),
            )
        }
    }
}

#[derive(Clone)]
pub struct Client {
    bedrock: BedrockClient,
}

impl Client {
    pub async fn new(region: &str, profile: Option<&str>) -> Result<Self> {
        let mut config_loader = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(aws_config::Region::new(region.to_string()));
        
        if let Some(profile_name) = profile {
            config_loader = config_loader.profile_name(profile_name);
        }
        
        let config = config_loader.load().await;
        let bedrock = BedrockClient::new(&config);
        Ok(Self { bedrock })
    }

    pub async fn stream_messages(
        &self,
        request: ApiRequest,
    ) -> Result<mpsc::Receiver<Result<StreamEvent>>> {
        let (tx, rx) = mpsc::channel(100);
        
        let mut req = self.bedrock
            .converse_stream()
            .model_id(&request.model);

        if let Some(system) = request.system {
            req = req.system(SystemContentBlock::Text(system));
        }

        for msg in request.messages {
            let role = match msg.role.as_str() {
                "user" => ConversationRole::User,
                "assistant" => ConversationRole::Assistant,
                _ => continue,
            };
            
            let mut message_builder = Message::builder().role(role);
            for content in msg.content {
                match content {
                    super::messages::ContentBlock::Text { text } => {
                        message_builder = message_builder.content(ContentBlock::Text(text));
                    }
                    super::messages::ContentBlock::ToolUse { id, name, input } => {
                        message_builder = message_builder.content(
                            ContentBlock::ToolUse(
                                aws_sdk_bedrockruntime::types::ToolUseBlock::builder()
                                    .tool_use_id(id)
                                    .name(name)
                                    .input(json_to_document(input))
                                    .build()?
                            )
                        );
                    }
                    super::messages::ContentBlock::ToolResult { tool_use_id, content } => {
                        message_builder = message_builder.content(
                            ContentBlock::ToolResult(
                                aws_sdk_bedrockruntime::types::ToolResultBlock::builder()
                                    .tool_use_id(tool_use_id)
                                    .content(aws_sdk_bedrockruntime::types::ToolResultContentBlock::Text(content))
                                    .build()?
                            )
                        );
                    }
                }
            }
            req = req.messages(message_builder.build()?);
        }

        if !request.tools.is_empty() {
            let mut tool_config = ToolConfiguration::builder();
            for tool in request.tools {
                tool_config = tool_config.tools(
                    Tool::ToolSpec(
                        aws_sdk_bedrockruntime::types::ToolSpecification::builder()
                            .name(tool.name)
                            .description(tool.description)
                            .input_schema(aws_sdk_bedrockruntime::types::ToolInputSchema::Json(json_to_document(tool.input_schema)))
                            .build()?
                    )
                );
            }
            req = req.tool_config(tool_config.build()?);
        }

        let response = req.send().await.context("Failed to call Bedrock ConverseStream")?;
        let mut stream = response.stream;

        tokio::spawn(async move {
            use aws_sdk_bedrockruntime::types::ConverseStreamOutput;
            loop {
                match stream.recv().await {
                    Ok(Some(event)) => {
                        let stream_event = match event {
                            ConverseStreamOutput::MessageStart(e) => {
                                StreamEvent::MessageStart { role: e.role().as_str().to_string() }
                            }
                            ConverseStreamOutput::ContentBlockStart(e) => {
                                if let Some(start) = e.start() {
                                    if start.is_tool_use() {
                                        let tool_use = start.as_tool_use().unwrap();
                                        StreamEvent::ContentBlockStart {
                                            index: e.content_block_index(),
                                            block_type: "tool_use".to_string(),
                                            tool_use_id: Some(tool_use.tool_use_id().to_string()),
                                            tool_name: Some(tool_use.name().to_string()),
                                        }
                                    } else {
                                        StreamEvent::ContentBlockStart {
                                            index: e.content_block_index(),
                                            block_type: "text".to_string(),
                                            tool_use_id: None,
                                            tool_name: None,
                                        }
                                    }
                                } else {
                                    continue;
                                }
                            }
                            ConverseStreamOutput::ContentBlockDelta(e) => {
                                if let Some(delta) = e.delta() {
                                    if delta.is_text() {
                                        StreamEvent::TextDelta {
                                            index: e.content_block_index(),
                                            text: delta.as_text().unwrap().to_string(),
                                        }
                                    } else if delta.is_tool_use() {
                                        StreamEvent::ToolInputDelta {
                                            index: e.content_block_index(),
                                            json: delta.as_tool_use().unwrap().input().to_string(),
                                        }
                                    } else {
                                        continue;
                                    }
                                } else {
                                    continue;
                                }
                            }
                            ConverseStreamOutput::ContentBlockStop(_) => {
                                StreamEvent::ContentBlockStop
                            }
                            ConverseStreamOutput::MessageStop(e) => {
                                let stop_reason_str = e.stop_reason().as_str().to_string();
                                StreamEvent::MessageStop {
                                    stop_reason: Some(stop_reason_str),
                                }
                            }
                            _ => continue,
                        };
                        
                        if tx.send(Ok(stream_event)).await.is_err() {
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        let _ = tx.send(Err(anyhow::anyhow!("Stream error: {:?}", e))).await;
                        break;
                    }
                }
            }
        });

        Ok(rx)
    }
}
