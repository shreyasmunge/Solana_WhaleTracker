# 🐋 Solana Whale Tracker

A full-stack web app to track Solana wallet balance in real time.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)


---

## 📌 What It Does
- Enter any Solana wallet address
- Fetches real-time SOL balance from Solana's public RPC API
- Converts SOL balance to USD using CoinGecko price API

---

## ⚙️ Tech Stack
| Layer | Tech |
|--|--|
| Frontend | React |
| Backend | Rust + Axum |
| Blockchain Data | Solana JSON-RPC API |
| Price Data | CoinGecko API |

---

## 🚀 Run Locally

### Backend (Rust)
```bash
cd backend
cargo run
# runs at http://localhost:8000
```

### Frontend (React)
```bash
cd frontend
npm install
npm start
# runs at http://localhost:3000
```

---

## 🔌 API
```
GET /wallet/:address
```
Returns SOL balance and USD value for any Solana wallet address.

---

## 💡 Test Wallets
No wallet? Use these:
```
9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
GsbwXfm9vXtKhFZmFdSsRhFGKwWAfrEZsEDMRgBjQnGA
```
Or find more at [solscan.io](https://solscan.io)

---

## 🗺️ Upcoming Features
- [ ] Last 5 transactions with amounts and timestamps
- [ ] 🐋 Whale alert for transactions over 500 SOL
- [ ] Auto-refresh every 30 seconds

---
