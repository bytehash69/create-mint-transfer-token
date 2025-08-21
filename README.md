# 🪙 Create Mint Transfer Token

A Solana program and TypeScript client for creating, minting, and transferring SPL tokens, built in 🦀 Rust and ⚡️ TypeScript using the [Anchor framework](https://book.anchor-lang.com/).

---

## ✨ Overview

**create-mint-transfer-token** provides a simple, secure way to create new SPL token mints, mint tokens to accounts, and transfer tokens on the Solana blockchain. It’s ideal for dApps, wallets, and platforms looking to integrate token management workflows.

---

## 🚀 Features

- 🆕 Create new SPL token mints on Solana
- 🔨 Mint tokens to wallets and accounts
- 🔄 Transfer SPL tokens between accounts
- 🛠 Built with Anchor for reliable, maintainable code
- 🟦 TypeScript client for easy integration with web and mobile apps

---

## 🛠 Getting Started

### 📋 Prerequisites

- 🦀 [Rust](https://www.rust-lang.org/tools/install)
- ☀️ [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- ⚓️ [Anchor](https://book.anchor-lang.com/getting_started/installation.html)
- 🟩 [Node.js](https://nodejs.org/) (for TypeScript client)

### ⬇️ Installation

```bash
git clone https://github.com/bytehash69/create-mint-transfer-token.git
cd create-mint-transfer-token
```

### 🏗 Build the Program

```bash
anchor build
```

### 🧪 Run Tests

```bash
anchor test
```

### 🛳 Deploy to Localnet

```bash
anchor deploy
```

---

## 📦 Usage

### 📝 Rust (On-Chain Program)

- Core logic lives in `programs/create-mint-transfer-token`.
- Implement instructions for mint creation, minting, and transfers.

### 🟦 TypeScript (Client SDK)

- The client (in `tests/` or `client/`) helps you interact with the on-chain program.
- Example scripts show how to create a mint, mint tokens, and transfer them.

---

## 🤝 Contributing

Contributions are welcome! Open issues or submit pull requests. For big changes, please discuss them first.

---

## 📄 License

Licensed under the MIT License.

---

## ✉️ Contact

Maintained by [bytehash69](https://github.com/bytehash69).

---

> ⚠️ **This project is not audited. Use at your own risk!**
