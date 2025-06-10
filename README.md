# Solana Name Registry

A simple name registry on the Solana blockchain. Each name (e.g., `name.sol`) is linked to an owner (wallet address), and users pay a fee to register names. Additional metadata can be stored, such as job title, bio, and more. Names can also be transferred between users.

## Components

- **Program (smart contract)** — written in [Anchor](https://book.anchor-lang.com/)
- **Frontend (React)** — *in development*

## Getting Started

> Prerequisites: [Solana CLI](https://solana.com/docs/intro/installation#install-the-solana-cli), [Anchor CLI](https://solana.com/docs/intro/installation#install-anchor-cli), Node.js, and Yarn

```bash
git clone https://github.com/o-tsaruk/solana-name-registry.git
cd solana-name-registry
anchor build
anchor test
```