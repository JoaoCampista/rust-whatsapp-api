use axum::{extract::{Json, Query, Extension}, response::IntoResponse, http::StatusCode};
use crate::models::WebhookData;
use std::collections::HashMap;

pub async fn post_webhook(Json(payload): Json<WebhookData>) -> impl IntoResponse {
    println!("{:?}", payload); // Isso irá imprimir o corpo da requisição deserializado
    
    // Iterar sobre cada entrada
    for entry in &payload.entry {
        // Iterar sobre cada mudança dentro de cada entrada
        for change in &entry.changes {
            // Agora, iterar sobre cada mensagem dentro de `value.messages`
            for message in &change.value.messages {
                // Finalmente, acessar o `body` dentro de `text`
                println!("Message Body: {}", message.text.body);
            }
        }
    }

    (StatusCode::OK, "Payload received")
}

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