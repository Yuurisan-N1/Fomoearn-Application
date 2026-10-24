<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=FomoEarn%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Watch%20and%20Earn%20%7C%20Auto%20Video%20Tasks%20%7C%20Attestation%20%7C%20Multi-Account&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Auto+Watch+Video+Tasks+%7C+Earn+USD+Per+Video;Attestation+Chain+%7C+keybox.xml+Required;Countdown+Per+Video+%7C+Never+Claims+Too+Early;probe+Mode+%7C+Read+Gates+Without+Claiming;Proxy+Support+%7C+Multi-Account"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-FomoEarn-111111"/>
  <img alt="multi-account" src="https://img.shields.io/badge/Multi--Account-Supported-111111"/>
  <img alt="build" src="https://github.com/Yuurisan-N1/Fomoearn-Bot/actions/workflows/build.yml/badge.svg"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>FomoEarn Bot</b> is a watch-and-earn automation client for the FomoEarn task feed.<br/>
  It performs a key exchange and attestation handshake per account, signs in, fetches the video task feed, counts down the required watch time before each claim, and credits the USD reward for every completed video, all running across multiple accounts with proxy support and a live countdown between cycles.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## Table of Contents

- [Requirements](#requirements)
- [Install Rust](#install-rust)
- [Installation](#installation)
- [Configuration](#configuration)
- [keybox.xml](#keyboxxml)
- [Running the Bot](#running-the-bot)
- [Run Modes](#run-modes)
- [Download Prebuilt Binary](#download-prebuilt-binary)
- [Features](#features)
- [File Structure](#file-structure)
- [Disclaimer](#disclaimer)

---

## Requirements

- Rust `1.70+` (includes `cargo`, only needed if building from source)
- Git

---

## Install Rust

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify:

```bash
rustc --version
```

### Windows

Download and run the installer from https://rustup.rs, then reopen the terminal and verify:

```cmd
rustc --version
```

### Termux (Android)

```bash
pkg update && pkg install proot-distro
proot-distro install ubuntu
proot-distro login ubuntu
```

Then inside Ubuntu:

```bash
apt update && apt install -y curl git build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## Installation

**Clone the repository:**

```bash
git clone https://github.com/Yuurisan-N1/Fomoearn-Bot.git
cd Fomoearn-Bot
```

---

## Configuration

### 1. Accounts (data.txt)

Fill `data.txt` with one account per line. Each line is `email|password`. Blank lines and lines starting with `#` are ignored. The password must not contain a second pipe character:

```
you@example.com|yourpassword
another@example.com|anotherpassword
```

### 2. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line (optional, leave empty to run without proxy):

```
socks5://user:pass@host:port
http://host:port
http://user:pass@host:port
```

Proxies are paired to accounts by position and rotate on every cycle. Proxy credentials are always masked in log output.

### 3. Bot Settings (config.json)

`sleep_seconds` controls how many seconds the bot waits between cycles. If `config.json` is missing, it is created automatically with a default of `3600` seconds.

---

## keybox.xml

The attestation chain is built from `keybox.xml`. This file must be placed in the same folder as the binary before running. Without it, the bot stops before the task feed with a config error.

Copy the example and fill in your own key and certificate chain:

```bash
cp keybox.example.xml keybox.xml
```

Then edit `keybox.xml` and replace the placeholder values with your actual ECDSA private key and its certificate chain.

---

## Running the Bot

### Using make (Linux / macOS)

```bash
make release    # optimized release build
make start      # build release and run immediately
make run        # build debug and run
make check      # check for errors without building
make fmt        # format the code
make clean      # remove all build artifacts
make size       # show release binary size
```

### Manual cargo build

```bash
cargo build --release
```

Then run:

```bash
# Linux / macOS / Termux
./target/release/fomoearn-bot

# Windows
.\target\release\fomoearn-bot.exe
```

Press `Ctrl+C` once to stop the bot cleanly.

---

## Run Modes

A bare run with no argument starts the full cycle. Two extra modes are available for debugging:

| Mode | Command | Description |
|---|---|---|
| run (default) | `./fomoearn-bot` | Full cycle: key exchange, sign in, watch tasks, claim rewards |
| probe | `./fomoearn-bot probe` | Read every gate and log the results without claiming anything |
| selftest | `./fomoearn-bot selftest` | Offline checks with no network calls |

---

## Download Prebuilt Binary

Prebuilt binaries are automatically compiled on every push to main via GitHub Actions across 3 platforms.

Download the latest binaries from the Actions page:
https://github.com/Yuurisan-N1/Fomoearn-Bot/actions/workflows/build.yml

Open the latest successful run and scroll to the Artifacts section.

| Artifact | Platform |
|---|---|
| `fomoearn-bot-linux-x86_64` | Linux x86_64 |
| `fomoearn-bot-windows-x86_64` | Windows x86_64 |
| `fomoearn-bot-macos-aarch64` | macOS Apple Silicon |

**Linux / macOS after downloading:**

```bash
chmod +x fomoearn-bot-linux-x86_64
./fomoearn-bot-linux-x86_64
```

**Windows:**

```bash
.\fomoearn-bot-windows-x86_64.exe
```

Place the binary in the same folder as `data.txt`, `proxy.txt`, `config.json`, and `keybox.xml` before running.

---

## Features

### Key Exchange and Attestation
Before signing in, the bot performs a key exchange handshake and builds an attestation envelope from the ECDSA key and certificate chain in `keybox.xml`. If the server rejects the attestation header, the log reports that the seal was rejected without exposing the request body.

### Auto Sign In
Each account signs in using the email and password from `data.txt`. A per-account device token and Android ID are derived and sent with every request to identify the session.

### Watch and Earn
After signing in, the bot fetches the next video task from the task feed. The required watch time is counted down in the terminal before the claim is submitted. A claim that arrives too early is refused by the server, so the countdown always runs in full before claiming. Each successfully credited video logs the USD amount earned. The loop continues until the feed returns no more tasks for the account.

### Countdown Per Video
Each video task includes a server-specified wait time. The bot displays a live `HH:MM:SS` countdown labeled `Next video is ready in` before submitting the claim.

### Log Colors
Green for a step that succeeded, yellow for a step that was skipped with a note, red for an account or the program stopping. Every refusal is a single red line with the reason and never the raw server response body.

### Multi Account
All accounts in `data.txt` are processed sequentially within every cycle. Each account runs its own key exchange and sign-in before the task loop.

### Proxy Support
Proxies are loaded from `proxy.txt` and paired to accounts by index. They rotate on every cycle. HTTP and SOCKS5 are both supported. Credentials are masked in log output. Running without proxies is fully supported.

### Auto Countdown
After all accounts complete a cycle, the bot displays a live `HH:MM:SS` countdown until the next cycle starts.

### Graceful Shutdown
Pressing `Ctrl+C` once sets a shutdown signal, prints the stopped message, and exits cleanly.

---

## File Structure

```text
Fomoearn-Bot/
├── .github/
│   └── workflows/
│       └── build.yml                    # CI: 3-platform matrix build
├── src/
│   ├── main.rs                          # Entry point, Ctrl+C handler, mode dispatch
│   ├── app/
│   │   ├── bootstrap.rs                 # Config load, keybox load, account and proxy setup
│   │   ├── dispatch.rs                  # Mode selector: run, probe, selftest
│   │   └── mod.rs
│   ├── domain/
│   │   ├── account.rs                   # Account model: email, password, device token, android ID
│   │   ├── task.rs                      # Task model: video ID, watch seconds, reward
│   │   └── mod.rs
│   ├── modes/
│   │   ├── run.rs                       # Full cycle: multi-account loop and countdown
│   │   ├── probe.rs                     # Probe mode: read gates without claiming
│   │   ├── selftest.rs                  # Selftest mode: offline checks
│   │   └── mod.rs
│   ├── net/
│   │   ├── client.rs                    # API client with cookie jar and session headers
│   │   ├── attest.rs                    # Attestation envelope builder
│   │   ├── der.rs                       # DER encoding utilities
│   │   ├── endpoint.rs                  # API endpoint definitions
│   │   ├── keybox.rs                    # keybox.xml loader and ECDSA key parser
│   │   ├── proxy.rs                     # Proxy pool loader and masker
│   │   ├── seal.rs                      # Seal header builder
│   │   └── mod.rs
│   ├── ops/
│   │   ├── runner.rs                    # Per-account runner: keyx, login, session, watch loop
│   │   ├── auth/
│   │   │   ├── keyx.rs                  # Key exchange handshake
│   │   │   ├── login.rs                 # Email/password sign-in
│   │   │   ├── session.rs               # Session token refresh
│   │   │   └── mod.rs
│   │   ├── user/
│   │   │   ├── dashboard.rs             # Profile and balance fetch
│   │   │   └── mod.rs
│   │   └── watch/
│   │       ├── list.rs                  # Fetch next video task from feed
│   │       ├── serve.rs                 # Count down watch time and submit claim
│   │       └── mod.rs
│   └── support/
│       ├── banner.rs                    # ASCII banner
│       ├── cfg/                         # Config loader, schema, setup struct
│       ├── consts/                      # App and network constants
│       ├── error.rs                     # Error types and refusal formatter
│       ├── log/                         # Log init, colored format, cash and countdown helpers
│       └── mod.rs
├── assets/
│   └── icon.ico                         # Windows binary icon
├── Cargo.toml                           # Project manifest and dependencies
├── Cargo.lock
├── build.rs                             # Build script (embeds icon into Windows binary)
├── Makefile                             # make targets: build, run, release, start, clean, size
├── config.json                          # Sleep duration between cycles
├── data.txt                             # Account credentials, email|password one per line
├── proxy.txt                            # Proxy list (optional)
├── keybox.xml                           # Attestation key and certificate chain (required)
├── keybox.example.xml                   # Attestation template to copy and fill
├── HOWTO.txt                            # Quick start guide
└── LICENSE                              # GNU AGPL v3.0
```

---

## Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>