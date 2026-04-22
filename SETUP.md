# DRide — Complete Setup Guide

This guide walks a **brand-new developer** through setting up the DRide project from scratch on **macOS** or **Windows**. Follow every step in order. Estimated time: 30–60 minutes.

---

## Table of Contents

1. [What You Need to Install](#1-what-you-need-to-install)
2. [macOS Setup](#2-macos-setup)
3. [Windows Setup](#3-windows-setup)
4. [Clone the Project](#4-clone-the-project)
5. [Install Project Dependencies](#5-install-project-dependencies)
6. [Set Up the Frontend Environment](#6-set-up-the-frontend-environment)
7. [Run the Frontend](#7-run-the-frontend)
8. [Set Up Phantom Wallet for Testing](#8-set-up-phantom-wallet-for-testing)
9. [Test the Full Flow](#9-test-the-full-flow)
10. [Troubleshooting](#10-troubleshooting)

---

## 1. What You Need to Install

| Tool | Purpose | Required Version |
|------|---------|-----------------|
| Git | Clone the project | Any recent |
| Node.js | Run the frontend | 18 or higher |
| Rust | Compile Solana programs | 1.75 or higher |
| Solana CLI | Interact with Solana network | 1.18 or higher |
| Anchor CLI | Build & deploy programs | 0.31 |
| Phantom | Browser wallet for testing | Latest |

> **Windows users:** Solana and Anchor do not run natively on Windows. You **must** use WSL 2 (Windows Subsystem for Linux). The Windows section below walks you through this step by step.

---

## 2. macOS Setup

Open **Terminal** (press `Cmd + Space`, type "Terminal", press Enter) and run the following commands one at a time.

### Step 1 — Install Homebrew (macOS package manager)

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

After it finishes, follow any instructions it prints (it may ask you to add Homebrew to your PATH). Then verify:

```bash
brew --version
```

### Step 2 — Install Git and Node.js

```bash
brew install git node
```

Verify:

```bash
git --version
node --version
npm --version
```

You should see version numbers for all three.

### Step 3 — Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, press `1` (default install). After it finishes:

```bash
source $HOME/.cargo/env
rustc --version
```

### Step 4 — Install Solana CLI

```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

When it finishes, it will print a line like:  
`export PATH="/Users/yourname/.local/share/solana/install/active_release/bin:$PATH"`

Copy that line and run it, then add it to your shell profile permanently:

```bash
# If you use zsh (default on modern macOS):
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# If you use bash:
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

Verify:

```bash
solana --version
```

### Step 5 — Install Anchor CLI

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.31.1
avm use 0.31.1
```

This takes several minutes. When done:

```bash
anchor --version
```

You should see `anchor-cli 0.31.1`.

---

## 3. Windows Setup

On Windows you must use **WSL 2** (Ubuntu inside Windows). All Solana/Anchor commands run inside WSL. The frontend can be run inside WSL or natively — either works.

### Step 1 — Enable WSL 2

Open **PowerShell as Administrator** (right-click the Start menu → Windows PowerShell (Admin)):

```powershell
wsl --install
```

Restart your computer when prompted.

After restarting, a terminal will open asking you to create a Ubuntu username and password. Fill these in — remember them.

Verify WSL is running:

```powershell
wsl --list --verbose
```

You should see Ubuntu listed with `VERSION 2`.

### Step 2 — Open Ubuntu terminal

Click Start → search "Ubuntu" → open it. All remaining steps in this section run **inside** this Ubuntu terminal.

### Step 3 — Update Ubuntu packages

```bash
sudo apt update && sudo apt upgrade -y
```

### Step 4 — Install build tools and Node.js

```bash
sudo apt install -y build-essential pkg-config libudev-dev curl git

# Install Node.js 20
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
```

Verify:

```bash
node --version
npm --version
git --version
```

### Step 5 — Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Press `1` for default install. Then:

```bash
source $HOME/.cargo/env
echo 'source $HOME/.cargo/env' >> ~/.bashrc
rustc --version
```

### Step 6 — Install Solana CLI

```bash
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

Add to PATH:

```bash
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
solana --version
```

### Step 7 — Install Anchor CLI

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.31.1
avm use 0.31.1
anchor --version
```

> **Note for Windows users:** You will edit code files using VS Code on Windows (not inside WSL). Install the [WSL extension for VS Code](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) and open your project from VS Code with `code .` from inside the Ubuntu terminal.

---

## 4. Clone the Project

Run this inside your Terminal (macOS) or Ubuntu (Windows):

```bash
git clone https://github.com/YOUR_USERNAME/DRide.git
cd DRide
```

Replace `YOUR_USERNAME/DRide` with the actual GitHub URL of the project.

---

## 5. Install Project Dependencies

### Anchor / Rust workspace dependencies

From the project root (`DRide/`):

```bash
yarn install
```

> If `yarn` is not found: `npm install -g yarn` then try again.

### Frontend (Next.js) dependencies

```bash
cd app
npm install
cd ..
```

---

## 6. Set Up the Frontend Environment

The frontend needs to know the program IDs of the two smart contracts deployed on Solana devnet. These are already deployed — you just need to tell the frontend where they are.

Inside the `app/` folder, create a file called `.env.local`:

```bash
# macOS / Linux / WSL:
cat > app/.env.local << 'EOF'
NEXT_PUBLIC_PROFILE_PROGRAM_ID=6h5cXVWNVtZ45XXQ5k1mHFQXtyeaUuYB7mUgvHc8kaCV
NEXT_PUBLIC_RIDE_PROGRAM_ID=GuiYvQDwxG3VfufPekrhvBSWbnhN1a9ReVE4U9Bwp5Ro
NEXT_PUBLIC_SOLANA_RPC_URL=https://api.devnet.solana.com
EOF
```

Or create it manually using any text editor with exactly this content:

```
NEXT_PUBLIC_PROFILE_PROGRAM_ID=6h5cXVWNVtZ45XXQ5k1mHFQXtyeaUuYB7mUgvHc8kaCV
NEXT_PUBLIC_RIDE_PROGRAM_ID=GuiYvQDwxG3VfufPekrhvBSWbnhN1a9ReVE4U9Bwp5Ro
NEXT_PUBLIC_SOLANA_RPC_URL=https://api.devnet.solana.com
```

Save it as `app/.env.local` (note the dot at the start — it must be exact).

---

## 7. Run the Frontend

```bash
cd app
npm run dev
```

You should see:

```
▲ Next.js 14.x.x
  - Local: http://localhost:3000
✓ Ready in Xms
```

Open **http://localhost:3000** in your browser (Chrome or Firefox with the Phantom extension installed).

---

## 8. Set Up Phantom Wallet for Testing

### Install Phantom

1. Go to https://phantom.app
2. Click **Download** and install the browser extension for Chrome, Firefox, or Brave
3. Open Phantom, click **Create a new wallet**
4. Write down your 12-word secret phrase somewhere safe (for testnet testing only)
5. Set a password

### Switch Phantom to Devnet

This app runs on Solana **Devnet** (a free test network). You must switch Phantom to Devnet:

1. Open Phantom extension
2. Click the **gear icon** (Settings) at the bottom right
3. Click **Developer Settings**
4. Click **Network**
5. Select **Devnet**

### Get free test SOL

You need test SOL (fake money) to pay transaction fees:

1. Copy your wallet address from Phantom (click your account name at the top)
2. Go to https://faucet.solana.com
3. Paste your address and click **Confirm Airdrop**
4. Repeat 1–2 times until you have at least 1 SOL

### Create a second wallet for testing

To test both Rider and Driver you need two wallets:

1. In Phantom, click your account name at the top
2. Click **Add / Connect Wallet → Create New Wallet**
3. This creates Account 2 with a separate address
4. Airdrop SOL to Account 2 as well (repeat steps above with Account 2's address)

---

## 9. Test the Full Flow

Open **two browser windows** (or one normal + one incognito).

**Window 1 — Rider:**
1. Go to http://localhost:3000
2. Connect Phantom with **Account 1**
3. Click **Rider**
4. Fill in your name and phone number → **Create profile** (approve transaction in Phantom)
5. Click **Where to?**
6. Enter a pickup address, drop-off address, and fare amount → **Request ride**
7. Approve the transaction(s) in Phantom
8. You'll be taken to the active ride page showing **Pending** status

**Window 2 — Driver:**
1. Go to http://localhost:3000
2. Connect Phantom with **Account 2** (switch accounts in Phantom first)
3. Click **Driver**
4. Fill in name, vehicle info, and phone → **Create profile** (approve transaction)
5. You'll see the pending ride request in the dashboard
6. Click **Accept** → approve transaction → you're taken to the active ride screen
7. Click **Picked up rider — Start ride** → approve
8. Click **Complete ride** → approve (this triggers the on-chain CPI)

**Back in Window 1:** The status updates automatically to Completed. No refresh needed.

---

## 10. Troubleshooting

### "anchor: command not found"
Run `source $HOME/.cargo/env` and try again. If still missing, re-run the Anchor install step.

### "solana: command not found"
Re-run the PATH export command for your shell and open a new terminal window.

### Phantom shows "Transaction simulation failed"
- Make sure Phantom is set to **Devnet** (not Mainnet)
- Make sure you have at least 0.1 SOL in your wallet
- Try the transaction again — devnet can be slow

### "airdrop request failed"
The devnet faucet rate-limits requests. Wait 60 seconds and try again, or use https://faucet.solana.com in a browser.

### Frontend shows blank / unstyled page
Stop the dev server (`Ctrl+C`), run `npm run dev` again, and **hard refresh** the browser (`Ctrl+Shift+R` or `Cmd+Shift+R`).

### "Program not found" error in the app
The `.env.local` file might be missing or have wrong values. Double-check it matches exactly what is shown in Step 6.

### WSL: can't open localhost:3000 in Windows browser
WSL 2 automatically forwards ports. Just open `http://localhost:3000` in your **Windows** browser (not inside WSL). If it doesn't work, try `http://127.0.0.1:3000`.

### Node version errors
Run `node --version`. If it shows lower than 18, install a newer version:
- macOS: `brew install node`
- WSL: repeat the Node.js install step from Step 4

---

## Summary of Commands

```bash
# After cloning and setting up .env.local:

cd DRide
yarn install          # Install Anchor test dependencies

cd app
npm install           # Install frontend dependencies
npm run dev           # Start the app at http://localhost:3000
```

That's it. The smart contracts are already deployed on Solana devnet, so you don't need to build or deploy anything — just run the frontend and use Phantom.
