# Zoya — Project Report
## Decentralized Ride-Sharing Application on Solana Blockchain

---

**Author:** zoya shaikh
**Date:** April 2026
**Network:** Solana Devnet

---

## 1. Executive Summary

Zoya is a decentralized ride-sharing application that replaces the traditional centralized backend of services like Uber with a pair of smart contracts running on the Solana blockchain. The system handles user registration, ride requests, ride lifecycle state transitions, and reputation tracking entirely on-chain, while keeping payment and real-time coordination off-chain for simplicity. The project demonstrates mastery of core Solana development concepts — Program Derived Addresses (PDAs), Anchor framework patterns, and Cross-Program Invocation (CPI) — and delivers a working end-to-end Web3 application.

---

## 2. Problem Statement

Traditional ride-sharing platforms such as Uber and Ola rely on centralized servers that introduce several well-known problems:

- **Opaque matching:** Users have no visibility into how drivers are matched or whether fares are computed fairly.
- **High platform fees:** Operators take 20–30% of each fare.
- **Single point of failure:** If the company's servers go down, the service stops.
- **Data silos:** Driver reputation, ratings, and history are locked to one platform and cannot be ported.
- **Censorship:** Drivers and riders can be deplatformed unilaterally.

Zoya addresses these limitations by storing the critical data — profiles, ride status, ratings — on the Solana blockchain. The result is a transparent, tamper-proof, portable, and uncensorable ride-sharing protocol.

---

## 3. Goals & Non-Goals

### Goals

- A working Solana dApp where riders and drivers can register on-chain.
- A ride lifecycle (request → accept → start → complete) fully managed on-chain.
- Demonstration of Cross-Program Invocation between two independent programs.
- A functional web frontend with wallet integration.
- Reproducible setup documentation for other developers.

### Non-Goals (intentionally scoped out of MVP)

- On-chain escrow / cryptocurrency payments (payment handled off-chain in MVP).
- Real-time GPS tracking (riders and drivers coordinate via phone numbers after match).
- Mobile applications (web-only MVP).
- Multi-network deployment (devnet only).

---

## 4. System Architecture

### 4.1 High-Level Diagram

```
┌───────────────────────────────────────────────────────────┐
│                  Next.js Web Application                   │
│                                                            │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────────┐  │
│  │ Phantom     │   │ Wallet      │   │ Anchor Client   │  │
│  │ Wallet      │──▶│ Adapter     │──▶│ (IDL + RPC)     │  │
│  └─────────────┘   └─────────────┘   └────────┬────────┘  │
└───────────────────────────────────────────────┼───────────┘
                                                │
                                                ▼ JSON-RPC
                   ┌──────────────────────────────────────┐
                   │       Solana Devnet Cluster          │
                   │                                      │
                   │   ┌─────────────────┐                │
                   │   │  zoya_profile   │                │
                   │   │  (Program 1)    │◀───CPI──┐     │
                   │   │                 │         │     │
                   │   │  • DriverProfile│         │     │
                   │   │  • RiderProfile │         │     │
                   │   └─────────────────┘         │     │
                   │                               │     │
                   │   ┌─────────────────┐         │     │
                   │   │   zoya_ride     │─────────┘     │
                   │   │   (Program 2)   │               │
                   │   │                 │               │
                   │   │  • RideRequest  │               │
                   │   └─────────────────┘               │
                   └──────────────────────────────────────┘
```

### 4.2 Why Two Programs?

A single monolithic program would have worked for this MVP, but splitting concerns into two programs has important benefits:

1. **Separation of concerns** — Profile state and ride state evolve independently. A future rating system can live in the profile program without touching ride logic.
2. **Independent upgrades** — Anchor programs can be upgraded only by their authority. Keeping them separate means we can iterate on ride mechanics without redeploying profile state.
3. **Real-world pattern** — Production Solana protocols like Metaplex split state and logic across multiple programs. Demonstrating CPI here shows understanding of how large Solana systems are built.

### 4.3 Cross-Program Invocation (CPI)

When a driver completes a ride, the `zoya_ride::complete_ride` instruction performs a CPI into `zoya_profile::increment_ride_count` to atomically bump the driver's `total_rides_completed` counter. Both state changes occur inside a single transaction — either both succeed or both revert. This preserves the invariant that "ride completion count" always matches "completed ride records".

---

## 5. Smart Contract Design

### 5.1 Program Derived Addresses

| Account | Program | Seeds | Purpose |
|---------|---------|-------|---------|
| `DriverProfile` | zoya_profile | `["driver_profile", driver_pubkey]` | One per driver wallet |
| `RiderProfile` | zoya_profile | `["rider_profile", rider_pubkey]` | One per rider wallet |
| `RideRequest` | zoya_ride | `["ride_request", rider_pubkey, ride_index]` | One per ride |

PDAs provide deterministic, collision-free addresses derived from seeds — the frontend can compute the same address the program uses without storing a mapping.

### 5.2 Account Schemas

**DriverProfile**
- `authority: Pubkey` — owner wallet
- `name: String` (max 32)
- `vehicle_info: String` (max 64)
- `phone: String` (max 16)
- `is_available: bool`
- `total_rides_completed: u64`
- `rating: u16` (stored ×100, e.g., 450 = 4.50 stars)
- `bump: u8`
- `created_at: i64`

**RiderProfile**
- `authority: Pubkey`
- `name: String` (max 32)
- `phone: String` (max 16)
- `total_rides: u64` (monotonic counter; also the ride_index seed)
- `rating: u16`
- `bump: u8`

**RideRequest**
- `rider: Pubkey`
- `driver: Pubkey` (default until accepted)
- `pickup_address: String` (max 128)
- `dropoff_address: String` (max 128)
- `fare_lamports: u64` (display value; not enforced on-chain in MVP)
- `status: RideStatus`
- `ride_index: u64`
- `bump: u8`
- `created_at`, `accepted_at`, `completed_at: i64`

### 5.3 Ride State Machine

```
          create_ride_request()
                  │
                  ▼
            ┌───────────┐
            │  PENDING  │────┐
            └─────┬─────┘    │
                  │          │
           accept_ride()     │ cancel_ride()
                  │          │
                  ▼          │
            ┌───────────┐    │
            │ ACCEPTED  │────┤
            └─────┬─────┘    │
                  │          │
            start_ride()     │ cancel_ride()
                  │          │
                  ▼          │
            ┌───────────┐    │
            │IN_PROGRESS│    │
            └─────┬─────┘    │
                  │          │
          complete_ride()    │
              (+ CPI)        │
                  ▼          ▼
            ┌───────────┐ ┌───────────┐
            │ COMPLETED │ │ CANCELLED │
            └───────────┘ └───────────┘
```

---

## 6. Frontend Design

### 6.1 Tech Choices

- **Next.js 14 (App Router)** — File-based routing, React Server Components where useful, fast dev server.
- **TypeScript** — Type safety across the Anchor IDL integration.
- **Tailwind CSS** — Rapid UI iteration.
- **@solana/wallet-adapter-react** — Standard wallet connection primitives.
- **@coral-xyz/anchor** — Generates typed program clients from the IDL.

### 6.2 Key Hooks

- `useProfileProgram` — encapsulates all interactions with `zoya_profile`.
- `useRideProgram` — encapsulates all interactions with `zoya_ride`, including passing the profile program ID through for CPI calls.
- `useRideState` — subscribes to on-chain account changes via `connection.onAccountChange` so the UI updates live when a ride's status changes, without polling.

### 6.3 Pages

| Route | Role | Purpose |
|-------|------|---------|
| `/` | Both | Landing, wallet connect, role selection |
| `/rider` | Rider | Dashboard, ride history |
| `/rider/request` | Rider | Form: pickup, drop-off, fare |
| `/rider/active` | Rider | Live status + driver contact info |
| `/driver` | Driver | List of pending rides |
| `/driver/active` | Driver | Start/Complete buttons, rider contact |

---

## 7. Implementation Plan & Timeline

| Day | Focus |
|-----|-------|
| 1 | Environment setup, Anchor workspace scaffolding |
| 2 | `zoya_profile` — state accounts + registration instructions + tests |
| 3 | `zoya_ride` — ride lifecycle instructions + CPI integration + tests |
| 4 | Deploy both programs to devnet, scaffold Next.js frontend |
| 5 | Frontend core — program clients, hooks, registration flow, rider/driver dashboards |
| 6 | Polish, error states, README, demo screenshots |

---

## 8. Verification & Testing Strategy

### 8.1 Unit tests (Anchor)
- Register driver → account fields populated correctly.
- Register rider → account fields populated correctly.
- Create ride request → `RideStatus::Pending`, `rider_profile.total_rides` incremented.
- Accept → `RideStatus::Accepted`, `driver` field set.
- Start → `RideStatus::InProgress`.
- Complete → `RideStatus::Completed` **AND** `DriverProfile.total_rides_completed` incremented (validates CPI).
- Cancel → `RideStatus::Cancelled`.
- Double-accept prevention — accepting an already-accepted ride fails with `InvalidRideStatus`.
- Authorization — only the ride's driver can complete it.

### 8.2 Manual end-to-end test

Using two Phantom wallet accounts in devnet mode:

1. Rider registers, requests a ride.
2. Driver registers in second account, sees the pending ride, accepts it.
3. Rider's screen shows status updating to "Accepted" within seconds (via WebSocket account subscription).
4. Driver clicks Start → Complete.
5. Run `anchor account zoya_profile.DriverProfile <PDA>` on the CLI — verify `total_rides_completed == 1`.

---

## 9. Deliverables

1. Two deployed Anchor programs on Solana devnet.
2. Anchor integration test suite (TypeScript).
3. Next.js web application with Phantom integration.
4. IDL files committed to `app/src/lib/idl/`.
5. Setup, usage, and architecture documentation (this report + README files).

---

## 10. Challenges & Design Decisions

### 10.1 Why no escrow?
An escrow system (rider deposits SOL, released to driver on completion) was considered but rejected for MVP because it adds two more PDAs, requires `invoke_signed` CPIs for native SOL transfers, and complicates cancellation refund logic. For a beginner-level project, off-chain payment keeps the smart contract surface small and the core educational value — on-chain matching + CPI — intact.

### 10.2 Why no GPS tracking?
Real-time GPS requires either an off-chain WebSocket service (Ably/Pusher) or high-frequency on-chain writes (prohibitively expensive). Removing GPS lets the rider and driver coordinate via phone numbers that surface once a match is confirmed — simpler and still functional.

### 10.3 Why `i64` for coordinates (in a future GPS version)?
Solana programs cannot use floating-point arithmetic (non-deterministic across validators). Coordinates would be stored as `lat * 1_000_000` to preserve six decimal places of precision (~0.11 m). Not in MVP but documented for future work.

### 10.4 Why Anchor rather than native Solana?
Anchor dramatically reduces boilerplate: account validation, serialization, and IDL generation are handled by macros. For a beginner-friendly project, it's the right trade-off.

---

## 11. Future Work

- On-chain escrow with SPL token (USDC) payments.
- Live GPS tracking via Ably or Solana program-owned location PDAs with rate limiting.
- Rating system — drivers and riders leave ratings post-ride, aggregated via CPI into profile programs.
- Dispute resolution contract with a neutral arbiter PDA.
- Deployment to Solana mainnet.
- Mobile app using React Native + Solana Mobile Stack.
- Surge pricing algorithm computed client-side but committed on-chain.

---

## 12. Conclusion

Zoya demonstrates that a working decentralized ride-sharing application can be built on Solana using only two small Anchor programs and a Next.js frontend. The project showcases the core primitives of Solana development — PDAs, Anchor account validation, and Cross-Program Invocation — in a practical, end-to-end application. The modular two-program design is extensible: payments, ratings, disputes, and governance can all be added as additional programs without rewriting existing code, exactly as happens in real-world Solana protocols.

---

## 13. References

- Solana Documentation — https://docs.solana.com
- Anchor Book — https://book.anchor-lang.com
- Solana Cookbook — https://solanacookbook.com
- Wallet Adapter — https://github.com/anza-xyz/wallet-adapter
- Metaplex (example of multi-program CPI architecture) — https://developers.metaplex.com
