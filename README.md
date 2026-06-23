# Aether

> A modern, keyboard-first music player for Linux.

🚧 **Status:** Early Development

Aether is an open-source music player built in Rust that unifies local music, YouTube, and SoundCloud into one fast, keyboard-driven experience. Designed with Wayland users in mind, Aether focuses on performance, modularity, and seamless desktop integration.

## Why Aether?

Most music players excel at one thing but compromise elsewhere.

* Local players often lack modern streaming support.
* Streaming clients rarely integrate well with Linux desktop workflows.
* Existing terminal music players can feel outdated or difficult to extend.

Aether aims to bridge that gap by providing a unified music experience that feels native to Linux while remaining lightweight, fast, and highly customizable.

## Goals

* Fast startup and low resource usage
* Native support for lossless audio formats
* Unified library for local and online music
* Keyboard-first workflow
* Linux-first design with Wayland support
* Modular architecture that's easy to extend

## Planned Features

### Playback

* Local audio playback
* FLAC, MP3, WAV, AAC, OGG, Opus, and ALAC support
* Gapless playback
* Queue management
* Shuffle and repeat modes
* ReplayGain support

### Library

* Automatic library scanning
* Metadata extraction
* Album artwork
* Favorites
* Play history
* Smart playlists

### Streaming

* YouTube playback
* SoundCloud playback
* Local caching of streamed content
* Internet radio

### Interfaces

* Command-line interface (CLI)
* Terminal user interface (TUI)
* Rofi/Wofi/Fuzzel integration
* Waybar integration
* MPRIS support
* Media key support

## Roadmap

### v0.1

* Local playback
* CLI
* Queue management
* FLAC support

### v0.2

* Library scanner
* Metadata database
* Search
* Playlists

### v0.3

* Background playback daemon
* IPC
* Persistent playback

### v0.4

* Terminal UI
* Album artwork
* Keyboard shortcuts

### v0.5

* YouTube support
* SoundCloud support
* Streaming cache

### v1.0

* MPRIS integration
* Waybar integration
* Smart playlists
* Plugin system
* Stable release

## Technology Stack

| Component     | Technology         |
| ------------- | ------------------ |
| Language      | Rust               |
| CLI           | Clap               |
| TUI           | Ratatui            |
| Audio         | Symphonia + CPAL   |
| Metadata      | Lofty              |
| Database      | SQLite             |
| Configuration | TOML + Serde       |
| Async Runtime | Tokio              |
| IPC           | Unix Sockets       |
| Streaming     | yt-dlp (initially) |

## Philosophy

Aether is built around a few simple principles:

* **Keyboard-first** — Every feature should be accessible without a mouse.
* **Linux-native** — Designed for Wayland, PipeWire, Hyprland, and modern Linux desktops.
* **Fast by default** — Startup, search, and playback should feel instantaneous.
* **Modular** — Components should be independent and easy to extend.
* **Source agnostic** — Local files and online sources should feel like one unified library.

## Contributing

Aether is in the early stages of development. Contributions, ideas, feature requests, and bug reports are welcome as the project evolves.

## License

This project is licensed under the MIT License.
