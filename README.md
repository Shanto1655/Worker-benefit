<img width="1919" height="1199" alt="Screenshot 2026-04-13 135451" src="https://github.com/user-attachments/assets/af70a885-e1bc-4c3d-8439-4b15927987eb" />

# 🛠️ Worker Benefit Smart Contract (Soroban)

## 📌 Project Description

The **Worker Benefit Smart Contract** is a decentralized solution built on the Stellar blockchain using Soroban. It enables organizations to manage and distribute benefits to workers in a transparent, secure, and tamper-proof manner.

Traditional systems for managing employee benefits are often centralized, opaque, and prone to manipulation. This project leverages blockchain technology to ensure fairness, accountability, and trust in benefit distribution.

---

## ⚙️ What It Does

This smart contract provides a simple yet effective mechanism to:

* 👷 Register workers on-chain
* 💰 Allocate benefits (points or token value) to workers
* 📤 Allow workers to securely claim their benefits
* 📊 Track benefit balances transparently

All operations are recorded on the blockchain, ensuring data integrity and auditability.

---

## ✨ Features

### 🔐 Secure Authentication

* Workers must authorize transactions before claiming benefits
* Prevents unauthorized access or misuse

### 📊 Transparent Ledger

* All benefit allocations are stored on-chain
* Anyone can verify balances and transactions

### 👷 Worker Management

* Easily register and manage workers
* Each worker has an independent benefit record

### ⚡ Efficient Storage Design

* Uses optimized Soroban storage patterns
* Lightweight and cost-efficient execution

### 🔁 Real-Time Benefit Updates

* Instant updates to worker balances
* Seamless claiming process

---

## 🔗 Deployed Smart Contract

**Contract ID:**
`CBUYSQTZCWGBHVFDG7ROTNIYLTIYVOCF2W7FSRBEULX2KIIY6IHOZKY6`

👉 You can interact with it using:

* Soroban CLI
* Stellar Laboratory
* Custom frontend integrations

---

## 🧱 Tech Stack

* 🦀 Rust (Smart Contract Development)
* 🌐 Soroban SDK
* ⭐ Stellar Blockchain

---

## 🚀 How It Works

1. **Add Worker**

   * Initializes a worker with zero benefits

2. **Assign Benefit**

   * Admin (or caller) assigns benefit value

3. **Claim Benefit**

   * Worker authenticates and claims assigned benefits

4. **Check Balance**

   * View current benefit balance anytime

---

## 📈 Use Cases

* 🏢 Employee reward systems
* 🧑‍🏭 Gig worker incentive tracking
* 🎓 Scholarship or stipend distribution
* 🏆 Performance-based bonus systems

---

## ⚠️ Current Limitations

* No admin access control (anyone can assign benefits)
* Benefits are stored as numbers (not real tokens yet)
* No expiration or time-lock mechanism

---

## 🔮 Future Improvements

* 🔑 Role-based admin system
* 🪙 Integration with Stellar tokens for real payouts
* ⏳ Benefit expiration logic
* 📱 Web/mobile dashboard for workers
* 📊 Analytics and reporting features

---

## 🧪 Example Usage (CLI)

```bash
# Add worker
soroban contract invoke \
  --id CBUYSQTZCWGBHVFDG7ROTNIYLTIYVOCF2W7FSRBEULX2KIIY6IHOZKY6 \
  --fn add_worker \
  --arg <WORKER_ADDRESS>

# Assign benefit
soroban contract invoke \
  --id CBUYSQTZCWGBHVFDG7ROTNIYLTIYVOCF2W7FSRBEULX2KIIY6IHOZKY6 \
  --fn assign_benefit \
  --arg <WORKER_ADDRESS> \
  --arg 100

# Check balance
soroban contract invoke \
  --id CBUYSQTZCWGBHVFDG7ROTNIYLTIYVOCF2W7FSRBEULX2KIIY6IHOZKY6 \
  --fn get_balance \
  --arg <WORKER_ADDRESS>
```

---

## 🏁 Conclusion

This project demonstrates how blockchain can simplify and secure worker benefit systems. With further enhancements, it can evolve into a full-scale decentralized HR and payroll solution.

---

## 📄 License

MIT License
