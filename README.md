# url-shortener

Projeto de treino em Rust — API de encurtador de URLs com `axum` e `tokio`.

## Contexto das branches

- `main`: mantém a versão com **Clean Architecture**.
- branch atual: versão focada em **Clean Code** (estrutura simples, direta e legível).

## Stack

| Crate | Uso |
|---|---|
| `axum 0.7` | framework HTTP |
| `tokio` | runtime async |
| `serde` + `serde_json` | serialização JSON |
| `rand` | geração de códigos aleatórios |
| `tower-http` | arquivos estáticos e CORS |

## Estrutura (clean code)

```
url-shortener/
├── Cargo.toml
├── src/
│   ├── main.rs          # entry point
│   ├── lib.rs           # composição da aplicação e bootstrap do servidor
│   ├── web.rs           # rotas + handlers HTTP
│   ├── shortener.rs     # regras de aplicação (encurtar e resolver)
│   ├── repository.rs    # persistência em memória (HashMap + RwLock)
│   ├── validation.rs    # validação de URL
│   └── code.rs          # geração de código curto
└── static/
    ├── index.html
    ├── css/style.css
    └── js/principal.js
```

## Rotas

| Método | Rota | Descrição |
|---|---|---|
| `GET` | `/` | health check |
| `POST` | `/api/shorten` | cadastra uma URL e retorna o código |
| `GET` | `/r/:code` | redireciona para a URL original (`302`) |
| `GET` | `/static/*` | interface web |

## Rodando o projeto

```bash
cargo run
```

Servidor em `http://localhost:3000`.

## Exemplo rápido

```bash
curl -X POST http://localhost:3000/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.google.com"}'
```
