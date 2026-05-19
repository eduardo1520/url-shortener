# url-shortener

Projeto de treino em Rust — API de encurtador de URLs com **Clean Architecture**, construída com `axum` e `tokio`. O armazenamento atual é em memória (`HashMap`), atrás de uma porta (`LinkRepository`) para facilitar troca por banco de dados depois.

## Objetivo

Praticar conceitos fundamentais de Rust em um contexto web real, com separação de responsabilidades:

- **Clean Architecture** — domínio, casos de uso, infraestrutura e entrega HTTP em camadas
- **Ownership e borrowing** — quando usar `String` vs `&str`
- **Lifetimes** — implícitos nos handlers e em funções de domínio
- **Concorrência segura** — `Arc<RwLock<T>>` no adaptador de persistência
- **Async/await** — runtime `tokio` na camada web

## Stack

| Crate | Uso |
|---|---|
| `axum 0.7` | framework HTTP |
| `tokio` | runtime async |
| `serde` + `serde_json` | serialização JSON |
| `rand` | geração de códigos aleatórios |
| `tower-http` | arquivos estáticos e CORS |

## Arquitetura

As dependências apontam **para dentro**: camadas externas conhecem as internas, nunca o contrário.

```
┌─────────────────────────────────────────┐
│  web/          handlers, DTOs, rotas    │  ← HTTP (axum)
├─────────────────────────────────────────┤
│  infrastructure/   MemoryLinkRepository │  ← adaptador
├─────────────────────────────────────────┤
│  application/   ShortenerService        │  ← casos de uso
│                 LinkRepository (trait)  │  ← porta
├─────────────────────────────────────────┤
│  domain/       validação, geração código│  ← regras puras
└─────────────────────────────────────────┘
```

| Camada | Módulo | Responsabilidade |
|---|---|---|
| **Domain** | `domain/` | Regras sem I/O: validar URL, gerar código alfanumérico |
| **Application** | `application/` | `ShortenerService` (encurtar / resolver); trait `LinkRepository` |
| **Infrastructure** | `infrastructure/` | Implementação em memória do repositório |
| **Web** | `web/` | Traduz HTTP ↔ casos de uso (status, JSON, redirect) |
| **Composition root** | `main.rs`, `lib.rs` | Monta dependências e inicia o servidor |

Fluxo ao encurtar uma URL:

1. `web/handlers` recebe o JSON e chama `ShortenerService::shorten`
2. O serviço valida no domínio, gera código único e persiste via `LinkRepository`
3. O handler devolve `201` com `code` e `short_url`

## Estrutura do projeto

```
url-shortener/
├── Cargo.toml
├── src/
│   ├── main.rs                          # composition root (só chama run())
│   ├── lib.rs                           # monta repo + service + router
│   ├── domain/
│   │   ├── validation.rs                # is_valid_url
│   │   └── code.rs                      # generate()
│   ├── application/
│   │   ├── ports.rs                     # trait LinkRepository
│   │   ├── service.rs                   # ShortenerService
│   │   └── errors.rs                    # ShortenError
│   ├── infrastructure/
│   │   └── memory_repository.rs         # HashMap + Arc<RwLock<>>
│   └── web/
│       ├── dto.rs                       # ShortenRequest
│       ├── handlers.rs                  # hello, shorten, redirect
│       └── router.rs                    # rotas e middlewares
└── static/
    ├── index.html
    ├── css/style.css
    └── js/principal.js
```

## Entry point: `main.rs` e `lib.rs`

Quando o projeto tem `src/main.rs` **e** `src/lib.rs`, o Cargo gera **dois alvos** no mesmo pacote:

| Alvo | Arquivo raiz | Resultado |
|---|---|---|
| **bin** | `main.rs` | executável (`cargo run`) |
| **lib** | `lib.rs` | biblioteca interna (crate `url_shortener`) |

### Quem é o entry point?

O **único** entry point do programa é o `main()` em `src/main.rs`.

Ao rodar `cargo run`, o sistema operacional carrega o binário e chama `main()`. O `lib.rs` **não** é entry point — ele vira uma biblioteca que o binário importa.

```
cargo run
    │
    ▼
compila lib.rs   →  crate url_shortener (domain, web, application, …)
    │
    ▼
compila main.rs  →  binário que linka com url_shortener
    │
    ▼
SO executa main() em main.rs
```

### Como o `main` usa o `lib`

O nome do pacote em `Cargo.toml` é `url-shortener`. No código Rust, hífens viram underscore: `url_shortener`.

```rust
// src/main.rs — só delega para a lib
#[tokio::main]
async fn main() {
    url_shortener::run().await;
}
```

```rust
// src/lib.rs — monta dependências e sobe o servidor
pub async fn run() {
    let repo = MemoryLinkRepository::new();
    let service = Arc::new(ShortenerService::new(repo));
    let app = web::router(service);
    // bind + axum::serve …
}
```

O `main` não declara `mod domain` nem `mod web` — isso fica organizado dentro da lib. Ele só inicia o runtime e chama `url_shortener::run()`.

### Por que separar?

Padrão **thin main, fat lib**:

- **`main.rs`** — ponto de entrada mínimo (runtime + uma chamada).
- **`lib.rs`** — lógica e módulos reutilizáveis; permite testes com `use url_shortener::…` sem subir HTTP, ou outros binários no futuro.

### O que faz o `#[tokio::main]`

O macro gera um `fn main()` **síncrono** que cria o runtime Tokio e executa `run().await` dentro dele. O entry point real continua sendo `main`; o `async` é só a API que você escreve.

**Resumo:** entry point = `main.rs`. `lib.rs` = biblioteca do mesmo projeto que o `main` chama — não substitui o `main` e não roda sozinha.

## Fluxo de execução

### Encurtar URL (`POST /api/shorten`)

```
Browser (index.html + principal.js)
    │  fetch POST /api/shorten  {"url": "https://..."}
    ▼
web/handlers::shorten          ← deserializa JSON, extrai State(service)
    │
    ▼
application::ShortenerService::shorten
    │  domain::validation::is_valid_url
    │  domain::code::generate  (loop se código já existir)
    │  infrastructure::insert
    ▼
201 Created  {"code": "…", "short_url": "/r/…"}
```

### Redirecionar (`GET /r/:code`)

```
Browser
    │  GET /r/k3x9mz
    ▼
web/handlers::redirect
    │
    ▼
application::ShortenerService::resolve
    │  infrastructure::get
    ▼
302 Found + Location: <url original>   (ou 404 se código não existir)
```

```
Browser  →  web (HTTP)  →  application (caso de uso)  →  domain (regras)
                                    ↓
                           infrastructure (HashMap)
```

## Rotas

| Método | Rota | Descrição |
|---|---|---|
| `GET` | `/` | health check |
| `POST` | `/api/shorten` | cadastra uma URL e retorna o código |
| `GET` | `/r/:code` | redireciona para a URL original (`302`) |
| `GET` | `/static/*` | interface web |

A interface em `http://localhost:3000/static/index.html` consome a API via `fetch`.

## Rodando o projeto

```bash
git clone <repo>
cd url-shortener
cargo run
```

O servidor sobe em `http://localhost:3000`.

## Exemplos

### Encurtar uma URL

```bash
curl -X POST http://localhost:3000/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.google.com"}'
```

Resposta:

```json
{
  "code": "k3x9mz",
  "short_url": "/r/k3x9mz"
}
```

### Usar o redirect

```bash
curl -L http://localhost:3000/r/k3x9mz
```

### URL inválida

```bash
curl -X POST http://localhost:3000/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "google.com"}'
```

Resposta `400 Bad Request`:

```json
{
  "error": "URL deve começar com http:// ou https://"
}
```

## Conceitos praticados

### Porta e adaptador (`LinkRepository`)

A camada de aplicação define o contrato; a infraestrutura implementa:

```rust
// application/ports.rs
pub trait LinkRepository: Send + Sync {
    fn get(&self, code: &str) -> Option<String>;
    fn contains(&self, code: &str) -> bool;
    fn insert(&self, code: String, url: String);
}
```

Para persistir em SQLite ou Redis, basta uma nova implementação em `infrastructure/` — sem alterar handlers nem domínio.

### `Arc<RwLock<HashMap>>` — estado compartilhado

O lock vive em `infrastructure/memory_repository.rs`, não nos handlers:

```rust
store: Arc<RwLock<HashMap<String, String>>>
```

`Arc` permite múltiplos donos entre threads. `RwLock` permite várias leituras simultâneas e uma escrita por vez. O guard é liberado ao sair de escopo (`Drop`).

### `String` vs `&str`

- DTOs e repositório usam `String` onde os dados vêm de fora (HTTP, armazenamento).
- Domínio e serviço usam `&str` ao validar ou consultar — empréstimo sem cópia.

### Lifetimes implícitos

```rust
// domain/validation.rs
pub fn is_valid_url(url: &str) -> bool { ... }
// equivale a:
pub fn is_valid_url<'a>(url: &'a str) -> bool { ... }
```

O compilador infere o lifetime automaticamente (*lifetime elision*).

## Próximos passos sugeridos

- Implementar `LinkRepository` com `sqlx` + SQLite (ou Redis)
- Substituir `expect` / `unwrap` no repositório por erros tipados (`thiserror`)
- Testes unitários em `domain/` e `application/` (sem subir HTTP)
- Testes de integração com `axum::test` na camada `web/`
- Expiração de URLs com `tokio::time` no serviço de aplicação
