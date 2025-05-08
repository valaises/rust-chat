use crate::messages::is_empty_string;
use crate::messages::is_none;
use serde::{Deserialize, Serialize};

use crate::image_utils::parse_image_b64_from_image_url_openai;
use crate::messages::{ContentArrayItem, Message, MessageContent, ToolCall};


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OContentArrayItemText {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

impl OContentArrayItemText {
    pub fn to_internal(&self) -> Result<ContentArrayItem, String> {
        ContentArrayItem::new(self.content_type.clone(), self.text.clone())
    }

    pub fn from_internal(item: ContentArrayItem) -> Self {
        Self {
            content_type: "text".to_string(),
            text: item.m_content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OContentArrayItemImageUrl {
    pub url: String,
    #[serde(default = "default_detail")]
    pub detail: String,
}

fn default_detail() -> String {
    "high".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OContentArrayItemImage {
    #[serde(rename = "type")]
    pub content_type: String,
    pub image_url: OContentArrayItemImageUrl,
}

impl OContentArrayItemImage {
    pub fn to_internal(&self) -> Result<ContentArrayItem, String> {
        let (image_type, _, image_content) = parse_image_b64_from_image_url_openai(&self.image_url.url)
            .ok_or(format!("Failed to parse image URL: {}", self.image_url.url))?;
        ContentArrayItem::new(image_type, image_content)
    }

    pub fn from_internal(item: ContentArrayItem) -> Self {
        let image_url = format!("data:{};base64,{}", item.m_type, item.m_content);
        Self {
            content_type: "image_url".to_string(),
            image_url: OContentArrayItemImageUrl {
                url: image_url,
                detail: default_detail()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum OContentArrayItem {
    OContentArrayItemText(OContentArrayItemText),
    OContentArrayItemImage(OContentArrayItemImage),
}

impl OContentArrayItem {
    pub fn to_internal(&self) -> Result<ContentArrayItem, String> {
        match self {
            OContentArrayItem::OContentArrayItemText(item) => item.to_internal(),
            OContentArrayItem::OContentArrayItemImage(item) => item.to_internal(),
        }
    }

    pub fn from_internal(item: ContentArrayItem) -> Result<Self, String> {
        if item.m_type == "text" {
            Ok(OContentArrayItem::OContentArrayItemText(OContentArrayItemText::from_internal(item)))
        } else if item.m_type.starts_with("image/") {
            Ok(OContentArrayItem::OContentArrayItemImage(OContentArrayItemImage::from_internal(item)))
        } else {
            Err(format!("Unsupported content type: {}", item.m_type))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum OMessageContent {
    ContentString(String),
    ContentArray(Vec<OContentArrayItem>),
}

impl OMessageContent {
    pub fn to_internal(&self) -> Result<MessageContent, String> {
        match self {
            OMessageContent::ContentString(content) => Ok(MessageContent::ContentString(content.clone())),
            OMessageContent::ContentArray(els) => {
                els.iter().map(|el| el.to_internal()).collect::<Result<Vec<_>, _>>().map(MessageContent::ContentArray)
            }
        }
    }

    pub fn from_internal(item: MessageContent) -> Result<Self, String> {
        match item {
            MessageContent::ContentString(content) => Ok(OMessageContent::ContentString(content)),
            MessageContent::ContentArray(items) => {
                let o_items = items.into_iter()
                    .map(OContentArrayItem::from_internal)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(OMessageContent::ContentArray(o_items))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OMessage {
    pub role: String,
    pub content: OMessageContent,
    #[serde(default, skip_serializing_if="is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(default, skip_serializing_if="is_empty_string")]
    pub tool_call_id: String,
}

impl OMessage {
    pub fn new(role: String, content: String) -> Self {
        Self {
            role,
            content: OMessageContent::ContentString(content),
            tool_calls: None,
            tool_call_id: "".to_string(),
        }
    }
    
    pub fn to_internal(&self) -> Result<Message, String> {
        let content = self.content.to_internal()?;
        Ok(Message {
            role: self.role.clone(),
            content,
            tool_calls: self.tool_calls.clone(),
            tool_call_id: self.tool_call_id.clone(),
        })
    }

    pub fn from_internal(item: Message) -> Result<Self, String> {
        let content = OMessageContent::from_internal(item.content)?;
        Ok(Self {
            role: item.role,
            content,
            tool_calls: item.tool_calls,
            tool_call_id: item.tool_call_id,
        })
    }
}