use serde::{Deserialize, Serialize};

use crate::image_utils::image_reader_from_b64string;
use crate::openai::OMessage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentArrayItem {
    pub m_type: String,
    pub m_content: String,
}

impl ContentArrayItem {
    pub fn new(m_type: String, m_content: String) -> Result<Self, String> {
        if !(m_type == "text") && !m_type.starts_with("image/") {
            return Err(format!("ContentArrayItem::new() received invalid type: {}", m_type));
        }
        if m_type.starts_with("image/") {
            let _ = image_reader_from_b64string(&m_content)
                .map_err(|e| format!("ContentArrayItem::new() failed to parse m_content: {}", e));
        }
        Ok(Self { m_type, m_content })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    ContentString(String),
    ContentArray(Vec<ContentArrayItem>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolFunction {
    pub arguments: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    pub function: ToolFunction,
    #[serde(rename = "type")]
    pub tool_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: MessageContent,
    #[serde(default, skip_serializing_if="is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(default, skip_serializing_if="is_empty_string")]
    pub tool_call_id: String,
}

pub fn is_none<T>(opt: &Option<T>) -> bool {
    opt.is_none()
}

pub fn is_empty_string(something: &String) -> bool {
    something.is_empty()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AnyMessage {
    Message(Message),
    OMessage(OMessage),
}

pub fn messages2openai(msgs: Vec<Message>) -> Result<Vec<OMessage>, String> {
    msgs.into_iter().map(|m| OMessage::from_internal(m)).collect()
}

pub fn any_messages2internal(msgs: Vec<AnyMessage>) -> Result<Vec<Message>, String> {
    msgs.into_iter().map(|m| match m {
        AnyMessage::Message(m) => Ok(m),
        AnyMessage::OMessage(m) => m.to_internal(),
    }).collect()
}