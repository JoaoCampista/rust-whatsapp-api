use axum::{extract::{Json, Query, Extension}, response::{IntoResponse, Response}, http::StatusCode};
use crate::models::{WebhookData, ChatRequest, MessageOpenAI};
use crate::controllers::openai::gpt_response;
use std::collections::HashMap;
use reqwest::Client;
use serde_json::json;

pub async fn post_webhook(Json(payload): Json<WebhookData>) -> impl IntoResponse {    
    println!("{:?}", payload); // Isso irá imprimir o corpo da requisição deserializado

    let mut text_body = &String::new();
    let mut from_number=  &String::new();
    let mut phone_number_id=  &String::new();

    for entry in &payload.entry {
        // Iterar sobre cada mudança dentro de cada entrada
        for change in &entry.changes {
            // Agora, iterar sobre cada mensagem dentro de `value.messages`
            for message in &change.value.messages {
                
                text_body = &message.text.body;
                from_number = &message.from;
                phone_number_id = &change.value.metadata.phone_number_id;
            }
        }
    }

    let chat_request = ChatRequest {
        messages: vec![
            MessageOpenAI {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            MessageOpenAI {
                role: "user".to_string(),
                content: text_body.to_string(),
            },
        ],
        model: "gpt-3.5-turbo".to_string(),
        frequency_penalty: Some(0.5),
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        max_tokens: Some(100),
        n: Some(1),
        presence_penalty: None,
        response_format: None,
        seed: Some(123456),
        stop: None,
        stream: None,
        temperature: Some(0.0),
        top_p: None,
        tools: None
    };

    let url = "https://api.openai.com/v1/chat/completions";
    let api_key = "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";

    match gpt_response(chat_request, url, api_key).await {
        Ok(response_text) => Response::builder()
            .status(200)
            .body(response_text)
            .expect("Failed to render response"),
        Err(err) => Response::builder()
            .status(500)
            .body(format!("Error: {}", err))
            .expect("Failed to render response"),
    }

}
// Adicione esta função para verificar a Webhook
pub async fn verify_webhook(
    Query(params): Query<HashMap<String, String>>,
    Extension(env_vars): Extension<HashMap<String, String>>,
) -> impl IntoResponse {
    let mode = params.get("hub.mode");
    let token = params.get("hub.verify_token"); 
    let challenge = params.get("hub.challenge");

    println!("{:?}", token);
    println!("{:?}", mode);
    println!("{:?}", challenge);

    if let (Some(mode), Some(token), Some(challenge)) = (mode, token, challenge) {
        if mode == "subscribe" && token == env_vars.get("VERIFY_TOKEN").unwrap_or(&String::new()) {
            println!("WEBHOOK_VERIFIED");
            // Para retornar um desafio como um inteiro, você precisa convertê-lo e retornar como uma resposta adequada
            match challenge.parse::<i32>() {
                Ok(challenge) => (StatusCode::OK, challenge.to_string()).into_response(),
                Err(_) => (StatusCode::BAD_REQUEST, "Invalid challenge".to_string()).into_response(),
            }
        } else {
            (StatusCode::FORBIDDEN, "").into_response()
        }
    } else {
        (StatusCode::BAD_REQUEST, "").into_response()
    }
}