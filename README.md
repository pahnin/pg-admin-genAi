# PG Admin GenAI

A **Postgres Admin Assistant** powered by LLMs with a **modern, minimal UI**.  
The app helps you query your Postgres database using natural language, with the model generating SQL queries and explanations.

Built in **Rust** with [Freya](https://github.com/marc2332/freya) for UI.

---

## ✨ Goals

* 🛠️ **Developer-friendly** – run everything locally without complex setup.
* 💰 **Minimal token usage** – optimized for local LLM inference instead of remote APIs.
* 🎨 **Modern, minimal UI** – clean interface for writing and running queries.

---

## 🚀 Running Locally

⚠️ **Note**: Configuring DB connection strings and OpenAI/LLM API endpoints via GUI is not implemented yet.  
For now, you must configure using `config.toml`.

### 1. Install Prerequisites

* [Rust & Cargo](https://www.rust-lang.org/tools/install)
* Local **Postgres** instance
* Local LLM server:
  * [Ollama](https://ollama.ai)
  * [LM Studio server](https://lmstudio.ai)  
  * or any server exposing an **OpenAI-compatible API**  
    (like `http://localhost:#{PORT}/v1/chat/completions`)

### 2. Run Postgres

Start Postgres locally with a database and tables that match your schema.  
Example: you can try the sample postgres dev server setup:

```bash
./tests/dev-db.sh
```

### 3. Run LLM server

Start an Ollama or compatible server locally.
Make sure it listens at:

```
http://localhost:1234/v1/chat/completions
```

Adjust the model name in [`src/llm.rs`](src/llm.rs) if you want to use a different one (default: `deepseek-coder-v2:latest`).

### 4. Update configuration

Update settings in `config.toml` to point to your DB server and LLM server:

```toml
# Which Postgres profile is active, you can add multiple profiles and select one as default
active_postgres = "local"

# Which LLM profile is active, you can add multiple profiles and select one as default
active_llm = "default"

[postgres_profiles.local]
host = "localhost"
port = 5432
user = "postgres"
password = "password"
dbname = "postgres"

[llm_profiles.default]
api_url = "http://localhost:1234/v1"
model = "deepseek-coder-v2-lite-instruct"
```

### 5. Run the app

```bash
cargo run
```

---

## Demo (27-08-2025)

[https://github.com/user-attachments/assets/7ad785b9-618d-409e-a243-aa23072deaec](https://github.com/user-attachments/assets/7ad785b9-618d-409e-a243-aa23072deaec)

---

## 🤝 Contributing

We welcome contributions!

1. Open an **issue** to discuss bugs, features, or ideas.
2. Submit a **pull request** with your changes.
3. Follow **semantic commit messages** ([Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)).

Examples:

* `feat(agent): add support for retrying on clarification errors`
* `fix(db): handle NULL values in query results`
* `chore(ui): improve error logging`

---

## 🧭 Roadmap

* [x] Configurable DB connection and LLM endpoint using config.toml
* [x] GitHub Actions CI with smoke tests
* [x] Major UI revamp with dropdowns, overlays, chat history
* [ ] GUI Configurable DB connection
* [ ] GUI Configurable LLM endpoint and model
* [-] Schema explorer in UI
* [ ] Tabbed SQL editor and saving SQL to file
* [ ] Reactive buttons (disable while async job is running)
* [ ] Context menus for tables (helper hooks based on AI)
* [ ] Context menus for individual cells (helper hooks based on AI)
* [ ] DB-level AI recommendations (index, constraints, normalization)
* [ ] Codegen (generate language code from SQL)
* [ ] Unit tests with mocks (instead of real Postgres)

---

