# Aether

![CI](https://github.com/DragonTTV/aether/actions/workflows/ci.yml/badge.svg)
![Latest Release](https://img.shields.io/github/v/release/DragonTTV/aether)
![License](https://img.shields.io/github/license/DragonTTV/aether)
![Downloads](https://img.shields.io/github/downloads/DragonTTV/aether/total)

Aether is a modern daemon-based terminal music player written in Rust.

Unlike traditional CLI music players, Aether separates playback from the command-line interface. A background daemon manages audio playback while lightweight clients communicate with it over IPC, enabling multiple frontends such as the CLI, TUI, GUI, and future integrations.

---

## Features

- Daemon-based playback architecture
- SQLite music library
- Metadata extraction
- Library scanning and search
- Queue management
- Shuffle and repeat
- Volume control
- MPRIS integration (Linux)
- systemd user service
- Native installer
- Cross-platform architecture

---

## Architecture

```text
 CLI / TUI / GUI
        │
        ▼
      IPC Layer
        │
        ▼
   Aether Daemon
        │
        ▼
      Player
        │
        ▼
       Audio
```

The daemon owns playback state, queue state, and audio output.

All clients communicate with the daemon over IPC, allowing multiple frontends to control playback simultaneously.

---

## Installation

### Linux

```bash
curl -fsSL https://raw.githubusercontent.com/DragonTTV/aether/main/scripts/install.sh | sh
```

Alternatively, download the latest release from the GitHub Releases page.

---

## Usage

Common playback commands:

```bash
aether play
aether pause
aether resume
aether next
aether previous
aether status
```

Library commands:

```bash
aether library list
aether library search "Artist"
```

---

## Platform Support

| Platform | Status |
|----------|--------|
| Linux | Stable |
| Windows | Planned |
| macOS | Planned |

Supported Linux architectures:

- x86_64
- aarch64

---

## Project Structure

| Binary | Description |
|--------|-------------|
| `aether` | Command-line client |
| `aetherd` | Background playback daemon |
| `aether-setup` | Installer, updater, repair and uninstall utility |

---

## Roadmap

### Completed

- [x] Daemon-based playback
- [x] SQLite music library
- [x] IPC communication
- [x] Queue management
- [x] MPRIS integration
- [x] Linux installer
- [x] Bootstrap installer
- [x] Playlists

### In Progress

- [ ] Windows Support

### Planned

- [ ] Terminal user interface
- [ ] Online metadata
- [ ] Online streaming
- [ ] macOS support
- [ ] Discord Rich Presence
- [ ] Last.fm integration
- [ ] Mobile companion application
- [ ] Graphical interface

---

## Building

### Requirements

- Rust (Stable)
- Cargo

Clone the repository:

```bash
git clone https://github.com/DragonTTV/aether.git
cd aether
```

Build:

```bash
cargo build --release
```

Run:

```bash
cargo run --bin aether
```

---

## License

Licensed under the MIT License.

See the `LICENSE` file for details.
