use dioxus::logger::tracing::{error, info};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::globals::{BACKEND_URL, API_KEY};
use crate::shared_state::{CompletionModel, SharedState, CompletionModelResponse};


pub async fn model_polling_service(_rx: UnboundedReceiver<()>) {
    let mut state = consume_context::<SharedState>();

    loop {
        match fetch_models().await {
            Ok(models) => {
                let mut models_state = state.models.write();
                models_state.models = models;
                models_state.is_loading = false;
                info!("Models updated, total: {}", models_state.models.len());
            },
            Err(err) => {
                let mut models_state = state.models.write();
                models_state.is_loading = false;
                models_state.error = Some(err.to_string());
                error!("Failed to fetch models: {}", err);
            }
        };
        TimeoutFuture::new(30_000).await;
    }
}

#[server]
async fn fetch_models() -> Result<Vec<CompletionModel>, ServerFnError> {
    let client = reqwest::Client::new();
    
    let response = client.get(format!("{}/models", BACKEND_URL))
        .header("Authorization", format!("Bearer {}", API_KEY))
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch models: {}", e)))?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(ServerFnError::new(format!(
            "API request failed with status: {}",
            response.status()
        )));
    }

    let models: CompletionModelResponse = response.json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse response: {}", e)))?;

    Ok(models.data)
}
