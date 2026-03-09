# FinanceVault

A local-first, fully encrypted personal finance app. No server, no cloud, no accounts. Your data never leaves your machine.

Built with Tauri, SvelteKit, and Rust.

## Features

- Encrypted local database — unlocked only with your passphrase
- Optional 2FA (TOTP)
- Expense tracking with categories and filters
- Subscription management — monthly and yearly, normalized to monthly cost
- Budget overview — income vs. expenses vs. subscriptions
- Portfolio tracking with Yahoo Finance price feed (read-only)
- PDF import for broker statements (Trade Republic, etc.)

## Requirements

- [Node.js](https://nodejs.org/) 20+
- [Rust](https://rustup.rs/) (stable)
- Xcode Command Line Tools (macOS: `xcode-select --install`)

## Start

```bash
# Install frontend dependencies (first time only)
cd frontend && npm install && cd ..

# Run in development mode
npm run tauri:dev
```

The app opens as a desktop window. On first launch, you create your passphrase to initialize the encrypted vault.

## Build

```bash
npm run tauri:build
```

The installer is placed in `src-tauri/target/release/bundle/`.

## Data location

The encrypted database is stored at:

- macOS: `~/Library/Application Support/com.financevault.app/vault.db`
- Linux: `~/.local/share/com.financevault.app/vault.db`
- Windows: `%APPDATA%\com.financevault.app\vault.db`

The database is encrypted with AES-256-GCM. Without the correct passphrase it cannot be read.
