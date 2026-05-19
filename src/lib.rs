mod code;
mod repository;
mod shortener;
mod validation;
mod web;

pub async fn run() {
    let repository = repository::MemoryRepository::new();
    let service = std::sync::Arc::new(shortener::ShortenerService::new(repository));
    let app = web::router(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("falha ao abrir porta 3000");

    println!("Servidor rodando em http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("servidor encerrado com erro");
}
