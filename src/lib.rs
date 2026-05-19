pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod web;

use std::sync::Arc;

use application::ShortenerService;
use infrastructure::MemoryLinkRepository;

pub async fn run() {
    let repo = MemoryLinkRepository::new();
    let service = Arc::new(ShortenerService::new(repo));
    let app = web::router(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("falha ao abrir porta 3000");

    println!("Servidor rodando em http://localhost:3000");

    axum::serve(listener, app).await.expect("servidor encerrado com erro");
}
