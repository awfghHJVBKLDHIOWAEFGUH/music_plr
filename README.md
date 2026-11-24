# GooseMusic TinySkia

This repository is an independent TinySkia fork of GooseMusic, currently based on GooseMusic v1.7.0 and using Iced's TinySkia renderer. The goal of this fork is to get TinySkia working without changing the rest of the app.

The v1.7.0 TinySkia migration is on [migration/upstream-v1.7.0-tiny-skia](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/migration/upstream-v1.7.0-tiny-skia).

## Changes

- Based on GooseMusic `v1.7.0`.
- Uses Iced `0.14`.
- Resolves `iced_tiny_skia 0.14.1` and `tiny-skia 0.11.4`.
- Enables the Iced `tiny-skia` feature.
- Removes the single explicit Iced `wgpu` feature.
- Keeps `image`, `svg`, `tokio`, `canvas`, `advanced`, `crisp`, `web-colors`, `thread-pool`, `x11`, and `wayland`.
- Updates `Cargo.lock`.
- Incorporates upstream changes through GooseMusic `v1.7.0`, including the InnerTube migration that replaces the Python/`ytmusicapi` search path and multi-author tracks.
- Retains the fork's GitHub Releases updater and cross-platform release packaging.

The v1.7.0 migration updates the renderer configuration, fork updater, release packaging, and documentation. WGPU-only package records were dropped from the lockfile; a few inactive WGPU-named records remain. No WGPU crate is in the active dependency graph.

The fork's release pipeline publishes Linux, macOS, and Windows assets with SHA-256 sidecars, macOS DMGs, and a Windows Inno Setup installer, and creates draft GitHub Releases.

## Memory usage

| Renderer | RSS | PSS |
|---|---:|---:|
| WGPU | 144.45 MB | 128.22 MB |
| TinySkia | 74.29 MB | 65.93 MB |

TinySkia reduced memory usage by approximately **48.6% RSS and 48.6% PSS** compared with WGPU in these measurements.

## Build and testing

The v1.7.0 migration branch passes `cargo metadata --locked --no-deps`, `cargo check --locked`, and the fork updater tests. An optimized release build (`cargo build --release --locked`) completed successfully on Linux x86_64 with LTO, `codegen-units = 1`, `panic = "abort"`, and stripping enabled.

The TinySkia renderer configuration remains the same as the v1.7.0 version that passed local runtime checks for audio playback, album artwork, mosaic rendering, and SVG rendering. Validation remains local, not a cross-platform test matrix; no FPS or speed improvement is claimed.

## Memory benchmarks

Local memory measurements on Linux:

| Build | RSS | PSS |
|---|---:|---:|
| Upstream GooseMusic | 144 MB | 172 MB |
| TinySkia fork | 72 MB | 72 MB |

Compared with the measured upstream build, the TinySkia build used approximately 50% less RSS and 58% less PSS.

These are local measurements, not universal performance guarantees.

## Cross-platform compatibility

For v1.7.0, cross-platform compatibility is expected but not confirmed. I have not tested Windows, macOS, or other Linux distributions for this release, so no measured compatibility rate is claimed.

The renderer change is still limited to configuration: I enabled Iced's TinySkia feature and removed the explicit Iced `wgpu` feature. I left the upstream audio, providers, playback, media controls, file handling, and platform-specific application code alone. The upstream changes through v1.7.0 come from GooseMusic upstream. The existing X11 and Wayland features are still enabled.

Basically, I changed how the interface is drawn, not how the app plays music or connects to providers. The build and runtime checks passed on Linux x86_64. Compatibility on Windows, macOS, and other Linux distributions is expected, but not confirmed.

## Known issue

Very long track titles can sometimes overlap the next row in the upright track list. It is a small visual issue and was left as-is.

## Build

You need the Rust toolchain.

```bash
git clone --branch migration/upstream-v1.7.0-tiny-skia https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr.git
cd music_plr
cargo build --release --locked
```

The binary is in `target/release/`.

The `v1.7.0-tiny-skia.1` release is being prepared on [GitHub Releases](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/releases). Release assets will be available there after the release workflow completes.

## Links

- [TinySkia v1.7.0 migration branch](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/migration/upstream-v1.7.0-tiny-skia)
- [v1.7.0 migration commit history](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/commits/migration/upstream-v1.7.0-tiny-skia)
- [Release v1.7.0-tiny-skia.1](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/releases/tag/v1.7.0-tiny-skia.1)
- [Original GooseMusic project](https://github.com/GooseOb/music_plr)
