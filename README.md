# Aether

> A modern, keyboard-first music player for Linux.

🚧 **Status:** Early Development

Aether is an open-source music player written in Rust. It is built around a daemon-first architecture, allowing multiple frontends to share a single playback engine while providing a fast, lightweight, and keyboard-driven music experience.

## Current Features

- Local audio playback
- Playback daemon (`aetherd`)
- Unix Domain Socket IPC
- Queue management
- Command-line interface (CLI)
- Automatic queue progression

## Technology Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| CLI | Clap |
| Audio | Rodio + Symphonia |
| Metadata | Lofty |
| Database | SQLite |
| Configuration | TOML + Serde |
| IPC | Unix Domain Sockets |

## Contributing

Aether is under active development. Contributions, bug reports, feature requests, and discussions are welcome.

## License

This project is licensed under the MIT License.