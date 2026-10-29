❯ # GooseMusic

GooseMusic is a desktop music player written in Rust using the Iced GUI framework. It supports online music search, streaming, downloads, local playback, playlists, lyrics and system media controls.

## Tiny Skia CPU Rendering

This branch changes Iced's rendering configuration to use Tiny Skia instead of WGPU for interface rendering.

Tiny Skia performs interface rendering on the CPU. It is an alternative for systems where the WGPU backend is unsuitable or CPU rendering is preferred.

This does not guarantee lower memory use, lower power consumption or better performance on every system. CPU rendering can increase CPU usage.

## Features

- Search supported music providers
- Stream and download tracks
- Play local audio files
- Manage playlists, albums and artists
- Playback queue and recently played tracks
- Lyrics and synced lyrics
- System media controls and media keys
- Downloads and library management

Feature availability depends on the application configuration and external services.

## Build

Requirements include Rust stable and the external tools needed by the music providers you use.

```sh
cargo build --release
```

The executable will be at `target/release/goosemusic`.

This branch enables Tiny Skia in the Cargo configuration and includes the corresponding dependency lockfile.

## Data locations

- Configuration: `~/.config/goosemusic`
- Application data: `~/.local/share/goosemusic`
- Cache: `~/.cache/goosemusic`

## Upstream

Original project: https://github.com/GooseOb/music_plr

See the upstream repository for complete documentation, dependencies and license information.
