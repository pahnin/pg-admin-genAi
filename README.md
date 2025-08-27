

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
For now, you must configure using config.toml

### 1. Install Prerequisites

* [Rust & Cargo](https://www.rust-lang.org/tools/install)
* Local **Postgres** instance
* Local LLM server:

  * [Ollama](https://ollama.ai)
  * or any server exposing an **OpenAI-compatible API** at `http://localhost:1234/v1/chat/completions`

### 2. Run Postgres

Start Postgres locally with a database and tables that match your schema.
Example (docker-compose snippet):

```yaml
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: yourpassword
      POSTGRES_DB: yourdb
    ports:
      - "5432:5432"
```

Update the connection string inside cargo.toml

### 3. Run LLM server

Start an Ollama or compatible server locally.
Make sure it listens at:

```
http://localhost:1234/v1/chat/completions
```

Adjust the model name in [`llm.rs`](src/llm.rs) if you want to use a different one (default: `deepseek-coder-v2:latest`).

### 4. Run the app

```bash
cargo run
```

---

## Demo ( 27-08-2025 ) 

https://github.com/user-attachments/assets/7ad785b9-618d-409e-a243-aa23072deaec

## 🖥️ UI

* Query editor: type a question in natural language
* “Execute query” button: generates SQL → executes on Postgres → shows results in a table
* Error handling: failed SQL or LLM issues are surfaced in the UI

The UI is built with **Freya** (a Rust-native UI framework).

---

## 🤝 Contributing

We welcome contributions!

1. Open an **issue** to discuss bugs, features, or ideas.
2. Submit a **pull request** with your changes.
3. Follow **semantic commit messages** ([Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)):

Examples:

* `feat(agent): add support for retrying on clarification errors`
* `fix(db): handle NULL values in query results`
* `chore(ui): improve error logging`

---

## 🧭 Roadmap

* [x] Configurable DB connection and LLM endpoint using config.toml
* [ ] GUI Configurable DB connection 
* [ ] GUI Configurable LLM endpoint and model
* [ ] Schema explorer in UI
* [x] Separate text field for Natual language text and SQL
* [ ] Tabbed SQL editor and saving SQL to file
* [ ] Reactive buttons ( disabling buttons while async job is running etc )
* [ ] Context menus for tables ( helper hooks based on AI )
* [ ] Context menus for individual cells ( helper hooks based on AI )
* [ ] DB level AI recommendations ( index recommendations, constraint recommendations, normalization recommendations )
* [ ] Codegen ( Generate code(in a given language) based on SQL
* [ ] Unit tests with mocks (instead of real Postgres)

---
