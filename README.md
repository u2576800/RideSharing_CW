# Zoya — Decentralized Ride-Sharing on Solana

Smart Contract:- [View Contract Details](https://explorer.solana.com/address/2B6yAr7mK8zAKDviAipYZZi1BoWFXdNNCfwDvsmWSMcM?cluster=devnet)


A blockchain-powered Uber-like ride-sharing application built on the Solana network. Zoya uses **two interoperating smart contracts** (Anchor programs) and a Next.js frontend with Phantom wallet integration. All ride data — profiles, requests, status transitions, ratings — is stored on-chain, making the system transparent, trustless, and tamper-proof.

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Tech Stack](#tech-stack)
4. [Prerequisites](#prerequisites)
5. [Setup for New Developers](#setup-for-new-developers)
6. [Building & Deploying Smart Contracts](#building--deploying-smart-contracts)
7. [Running the Frontend](#running-the-frontend)
8. [How to Use Zoya](#how-to-use-DRide)
9. [Project Structure](#project-structure)
10. [Troubleshooting](#troubleshooting)
11. [Contributing](#contributing)

---

## Project Overview

Zoya replaces Uber's centralized backend with two on-chain programs that handle driver/rider profiles, ride matching, and ride lifecycle management. Payments happen off-chain (cash or direct wallet transfer) to keep the MVP simple. The application demonstrates real-world Solana development patterns including:

- **Program Derived Addresses (PDAs)** for deterministic account addressing
- **Cross-Program Invocation (CPI)** between two independent programs
- **Anchor framework** idioms for account validation and state management
- **Wallet-based authentication** using Solana Wallet Adapter

---

## Architecture

### High-Level Diagram

```
┌────────────────────────────────────────────────────────────┐
│                   Next.js Frontend                          │
│         (Wallet Adapter + Phantom + Anchor Client)          │
└────────────────────────────┬───────────────────────────────┘
                             │ RPC
                             ↓
          ┌─────────────────────────────────────┐
          │       Solana Devnet Cluster         │
          │  ┌───────────────┐  ┌────────────┐  │
          │  │ zoya_profile  │─CPI→│ zoya_ride │ │
          │  │  (Program 1)  │  │(Program 2) │  │
          │  └───────────────┘  └────────────┘  │
          └─────────────────────────────────────┘
```

### Two-Program Design

**`zoya_profile`** — Manages driver and rider identities.
- `register_driver` — Creates a `DriverProfile` PDA.
- `register_rider` — Creates a `RiderProfile` PDA.
- `increment_ride_count` — Called via CPI by `zoya_ride` when a ride completes.

**`zoya_ride`** — Manages the ride lifecycle state machine.
- `create_ride_request` — Rider posts a request.
- `accept_ride` — Driver accepts.
- `start_ride` — Driver marks pickup.
- `complete_ride` — Driver marks drop-off → CPI into `zoya_profile` to bump `total_rides_completed`.
- `cancel_ride` — Either party cancels.

### Ride State Machine

```
PENDING → ACCEPTED → IN_PROGRESS → COMPLETED
   ↓          ↓
CANCELLED  CANCELLED
```

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Smart Contracts | Rust, Anchor Framework |
| Blockchain | Solana (devnet) |
| Frontend | Next.js 14 (App Router), TypeScript, Tailwind CSS |
| Wallet | Phantom (via `@solana/wallet-adapter-react`) |
| RPC Client | `@coral-xyz/anchor`, `@solana/web3.js` |
| Testing | Mocha + Chai (TypeScript) |

---

## Prerequisites

Install the following on your development machine before starting:

| Tool | Minimum Version | Install Command / Link |
|------|-----------------|------------------------|
| Rust | 1.75+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Solana CLI | 1.18+ | `sh -c "$(curl -sSfL https://release.solana.com/stable/install)"` |
| Anchor CLI | 0.30+ | `cargo install --git https://github.com/coral-xyz/anchor avm --locked && avm install latest && avm use latest` |
| Node.js | 18+ | https://nodejs.org |
| Yarn or npm | latest | Comes with Node.js |
| Phantom Wallet | Browser extension | https://phantom.app |

Verify installations:
```bash
rustc --version
solana --version
anchor --version
node --version
```

---

## Setup for New Developers

Follow these steps to get Zoya running locally.

### Step 1 — Clone and enter the project
```bash
git clone <your-repo-url> Zoya_project
cd Zoya_project
```

### Step 2 — Configure Solana CLI for devnet
```bash
solana config set --url devnet
solana-keygen new --outfile ~/.config/solana/id.json   # Skip if you already have a keypair
solana airdrop 2                                        # Get free devnet SOL
solana balance                                          # Should show ~2 SOL
```

### Step 3 — Install smart contract dependencies
```bash
# From project root
yarn install          # Installs test dependencies (Mocha, Chai, TypeScript)
```

### Step 4 — Install frontend dependencies
```bash
cd app
npm install
cd ..
```

### Step 5 — Set up frontend environment variables
Create `app/.env.local`:
```bash
NEXT_PUBLIC_PROFILE_PROGRAM_ID=<filled in after first deploy>
NEXT_PUBLIC_RIDE_PROGRAM_ID=<filled in after first deploy>
NEXT_PUBLIC_SOLANA_RPC_URL=https://api.devnet.solana.com
```

---

## Building & Deploying Smart Contracts

### Generate program keypairs (first time only)
```bash
anchor keys list
```
Copy both program IDs into:
1. `programs/zoya_profile/src/lib.rs` → `declare_id!("...")`
2. `programs/zoya_ride/src/lib.rs`    → `declare_id!("...")`
3. `Anchor.toml` under `[programs.devnet]`

### Build
```bash
anchor build
```
This compiles both programs and generates IDL files in `target/idl/`.

### Run tests
```bash
anchor test
```
All tests run against a local validator. Expected output: all test cases pass including the CPI integration test.

### Deploy to devnet
```bash
anchor deploy --provider.cluster devnet
```
After deploy, copy the IDL to the frontend:
```bash
cp target/idl/zoya_profile.json app/src/lib/idl/
cp target/idl/zoya_ride.json    app/src/lib/idl/
```

Then update `app/.env.local` with the deployed program IDs.

---

## Running the Frontend

```bash
cd app
npm run dev
```

Open http://localhost:3000 in a browser that has the Phantom extension installed. In Phantom settings, switch to **Devnet** network.

---

## How to Use Zoya

### As a Rider

1. Open the app and click **Connect Wallet**. Approve Phantom's connection prompt.
2. Choose **I'm a Rider** on the landing page.
3. Fill in your name and phone number → click **Register**. Approve the transaction in Phantom.
4. From the rider dashboard, click **Request a Ride**.
5. Enter the pickup address, drop-off address, and proposed fare (in SOL).
6. Submit. You'll see a pending ride status that updates automatically when a driver accepts.
7. Once accepted, the driver's phone number appears — call or text them to coordinate pickup.
8. Wait for the driver to mark the ride **Started** → **Completed**. Pay off-chain as agreed.

### As a Driver

1. Connect Phantom (Devnet mode) and select **I'm a Driver**.
2. Register your name, vehicle info (e.g., "Toyota Camry — KA01AB1234"), and phone number.
3. The driver dashboard lists all pending ride requests with pickup/drop-off addresses and fare.
4. Click **Accept** on a ride you want to take.
5. On the active-ride screen, the rider's phone appears. Call to coordinate.
6. When you pick them up, click **Start Ride**.
7. On drop-off, click **Complete Ride**. This triggers the on-chain CPI that increments your `total_rides_completed`.

### Testing with two wallets

Open two browser profiles (or use Phantom's account-switcher). In the first profile, register as a rider and request a ride. In the second profile, register as a driver and accept it. You can observe the status transitions in real time because the frontend subscribes to on-chain account changes via a WebSocket.

---

## Project Structure

```
Zoya_project/
├── README.md                # This file
├── REPORT.md                # Full project report
├── Anchor.toml              # Anchor workspace config
├── Cargo.toml               # Rust workspace
├── programs/
│   ├── zoya_profile/        # Smart contract 1: profiles
│   └── zoya_ride/           # Smart contract 2: ride lifecycle
├── tests/                   # Anchor integration tests (TypeScript)
└── app/                     # Next.js frontend
    ├── README.md            # Frontend-specific docs
    └── src/
        ├── app/             # Pages (App Router)
        ├── components/
        ├── hooks/
        ├── lib/
        └── providers/
```

---

## Troubleshooting

**`anchor build` fails with "program ID mismatch"**
Make sure the ID in `declare_id!()` matches the one in `Anchor.toml` under `[programs.devnet]`. Run `anchor keys list` to see the correct values.

**`solana airdrop 2` fails with rate limit**
Devnet faucet limits airdrops. Wait a few minutes or use https://faucet.solana.com in a browser.

**Phantom doesn't show the app's network**
Open Phantom → Settings → Developer Settings → Change Network → Devnet.

**Frontend says "Program not found"**
The program IDs in `app/.env.local` don't match what's deployed. Re-run `anchor deploy` and update the env vars.

**CPI transaction fails with "Instruction caller mismatch"**
Ensure `zoya_ride/Cargo.toml` has `zoya_profile = { path = "../zoya_profile", features = ["cpi"] }`.

---

## Contributing

1. Fork the repo.
2. Create a feature branch: `git checkout -b feat/my-feature`.
3. Ensure `anchor test` passes and the frontend builds (`npm run build` in `app/`).
4. Open a pull request describing the change.

---

## License

MIT
