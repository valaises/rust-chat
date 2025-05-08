use reqwest::Client;
use futures::{Stream, StreamExt};
use async_stream::stream;
use bytes::Bytes;
use serde::Serialize;
use crate::chat_completions::comp_stream_utils::ChunkStream;
use crate::globals::{API_KEY, BACKEND_URL};
use crate::openai::OMessage;


#[cfg(feature = "server")]
async fn post_request_and_yield_chunks(
    url: String, post: CompletionRestreamerPost
) -> Result<ChunkStream<impl Stream<Item = reqwest::Result<Bytes>> + Send + Sync>, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", API_KEY))
        .json(&post)
        .send().await?;

    let stream = response.bytes_stream();
    Ok(ChunkStream::new(stream))
}

#[cfg(feature = "server")]
pub async fn get_completion_stream<'a>(
    post: CompletionRestreamerPost
) -> Result<impl Stream<Item = Result<Bytes, String>> + Send + Sync + 'a, String> {
    let url = format!("{}/chat/completions", BACKEND_URL);

    Ok(stream! {
        match post_request_and_yield_chunks(url.clone(), post.clone()).await {
            Ok(mut stream) => {
                while let Some(chunk) = stream.next().await {
                    yield chunk.map_err(|e| e.to_string());
                }
            }
            Err(e) => yield Err(e.to_string()),
        }
    })
}

pub async fn get_completion_not_stream(
    post: CompletionRestreamerPost
) -> Result<Bytes, String> {
    let url = format!("{}/chat/completions", BACKEND_URL);

    let client = Client::new();
    let response = client.post(&url)
        .json(&post)
        .header("Authorization", format!("Bearer {}", API_KEY))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.bytes().await.map_err(|e| e.to_string())
}

#[derive(Serialize, Clone)]
pub struct CompletionRestreamerPost {
    pub model: String,
    pub messages: Vec<OMessage>,
    #[serde(default)]
    pub tools: Option<serde_json::Value>,
    #[serde(default)]
    pub tool_choice: Option<bool>,
    pub stream: bool,

    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub n: Option<i16>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub top_n: Option<f32>,
    #[serde(default)]
    pub stop: Option<Vec<String>>,
}
