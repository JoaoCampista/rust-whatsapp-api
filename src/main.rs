use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use std::{net::SocketAddr, env, collections::HashMap};
use dotenv::dotenv;

// Ajuste os caminhos de importação conforme a nova estrutura do seu projeto
mod models;
mod controllers;

// Importando as funções dos módulos de controladores
use crate::controllers::webhook::{post_webhook, verify_webhook};

#[tokio::main]
async fn main() {
    dotenv().ok(); // Carrega as variáveis de ambiente do arquivo .env

    // Configuração do servidor e rotas
    let app: Router = Router::new()
        .route("/webhook", get(verify_webhook).post(post_webhook)) // Adicionando corretamente a rota GET e POST para /webhook
        .layer(Extension(env::vars().collect::<HashMap<String, String>>())); // Passa as variáveis de ambiente para o estado do aplicativo

    // Define o endereço de escuta do servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on http://{}", addr);

    // Inicia o servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start the server");
}
