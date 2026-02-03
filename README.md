# ghostty-web

A web-based terminal emulator that runs an interactive shell directly in the browser. This full-stack application connects a browser-based terminal UI with a native shell running on the backend server.

## Tech Stack

**Backend (Rust)**
- Axum - Async web framework
- Tokio - Async runtime
- portable-pty - Cross-platform PTY management

**Frontend (TypeScript + React)**
- React 18
- ghostty-web - Terminal emulator library
- Vite - Build tool

## Prerequisites

- Rust toolchain
- Node.js and npm

## Installation

```bash
# Backend
cd backend
cargo build

# Frontend
cd frontend
npm install
```

## Running

Start both servers in separate terminals:

```bash
# Terminal 1: Backend server (http://127.0.0.1:3001)
cd backend
cargo run

# Terminal 2: Frontend dev server (http://localhost:5173)
cd frontend
npm run dev
```

## Production Build

```bash
# Frontend
cd frontend
npm run build

# Backend
cd backend
cargo build --release
```

## Project Structure

```
vim-web/
├── backend/
│   └── src/
│       ├── main.rs    # Server entry point
│       ├── ws.rs      # WebSocket handler
│       └── pty.rs     # PTY management
└── frontend/
    └── src/
        ├── main.tsx
        ├── App.tsx
        └── components/
            └── Terminal.tsx
```

## Features

- Real-time WebSocket communication
- PTY management with configurable shell
- Terminal emulation using Ghostty
- Auto-resizing terminal
- Dark theme with customizable styling
