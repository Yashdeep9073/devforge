# DevForge

A lightweight, high-performance developer toolbox built with Rust.

DevForge aims to provide essential developer utilities in a single fast desktop application without the memory overhead of Electron-based tools.

## Vision

Modern developer tools often consume hundreds of megabytes of RAM while providing features that many developers never use.

DevForge focuses on:

- Performance
- Low memory usage
- Fast startup times
- Native desktop experience
- Developer productivity

## Planned Features

### API Client

- GET, POST, PUT, PATCH, DELETE requests
- Request headers
- JSON request body
- Response viewer
- Request history
- Collection management

### Developer Utilities

- JSON Formatter
- JWT Decoder
- Base64 Encoder / Decoder
- UUID Generator
- Environment Variables

### Database Tools

- PostgreSQL Explorer
- Redis Explorer
- SQL Query Runner

### Advanced Features

- WebSocket Client
- Team Collections
- Cloud Synchronization
- AI-assisted API generation

## Tech Stack

### Core

- Rust
- Tokio
- Reqwest
- Serde

### Backend

- Axum
- SQLx
- PostgreSQL
- Redis

### Desktop UI

- Iced

### Infrastructure

- Docker
- Tracing

## Project Goals

- Build a production-grade Rust application
- Learn systems programming concepts through real-world development
- Create a practical alternative to bloated developer tools
- Explore high-performance desktop software architecture

## Current Progress

### Phase 1

- [x] Project initialization
- [x] Cargo setup
- [x] HTTP client foundation
- [x] Async runtime integration

### Phase 2

- [ ] HTTP methods abstraction
- [ ] Request models
- [ ] Header management
- [ ] JSON request body support

### Phase 3

- [ ] SQLite persistence
- [ ] Request history
- [ ] Collection management

### Phase 4

- [ ] Desktop UI with Iced
- [ ] Multi-tab request interface
- [ ] Response visualization

## Why Rust?

DevForge is built in Rust to achieve:

- Memory safety
- Excellent performance
- Low resource consumption
- Cross-platform support
- Strong developer tooling

## Getting Started

Clone the repository:

```bash
git clone https://github.com/<your-username>/devforge.git
cd devforge
```

Run the project:

```bash
cargo run
```

Build a release version:

```bash
cargo build --release
```

## Roadmap

- API Client
- JSON Tools
- Authentication Helpers
- Database Tools
- WebSocket Support
- Plugin System
- Cloud Sync

## License

MIT

---

Built while learning Rust and systems programming through real-world software development.
