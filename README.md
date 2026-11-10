# GooseMusic TinySkia

This repository is an independent TinySkia fork of GooseMusic, currently based on GooseMusic v1.5.5 and using Iced's TinySkia renderer. The goal of this fork is to get TinySkia working without changing the rest of the app.

The v1.5.5 renderer changes are on [release-prep/tiny-skia-v1.5.5](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/release-prep/tiny-skia-v1.5.5).

## Changes

- Based on GooseMusic `v1.5.5`.
- Uses Iced `0.14`.
- Resolves `iced_tiny_skia 0.14.1` and `tiny-skia 0.11.4`.
- Enables the Iced `tiny-skia` feature.
- Removes the single explicit Iced `wgpu` feature.
- Keeps `image`, `svg`, `tokio`, `canvas`, `advanced`, `crisp`, `web-colors`, `thread-pool`, `x11`, and `wayland`.
- Updates `Cargo.lock`.
- Incorporates the upstream v1.5.2–v1.5.5 changes, including the InnerTube migration that replaces the Python/`ytmusicapi` search path and multi-author tracks.
- Retains the fork's GitHub Releases updater and cross-platform release packaging.

The v1.5.5 migration changes `Cargo.toml` and `Cargo.lock` for the renderer, plus the fork updater, release packaging, and documentation. The lockfile diff is 13 insertions and 315 deletions. WGPU-only package records were dropped; a few inactive WGPU-named records remain. No WGPU crate is in the active dependency graph or the release binary.

The fork's release pipeline publishes Linux, macOS, and Windows assets with SHA-256 sidecars, macOS DMGs, and a Windows Inno Setup installer, and creates draft GitHub Releases.

## Memory usage

In the recorded memory comparison, normal GooseMusic used **122 MiB RSS and 122 MiB PSS**, while this TinySkia fork used **44 MiB RSS and 44 MiB PSS**. That's a reduction of approximately **64%** in both reported measurements.

## Build and testing

The v1.5.5 migration branch passes `cargo metadata --locked --no-deps`, `cargo check --locked`, and the fork updater tests. An optimized release build (`cargo build --release --locked`) completed successfully on Linux x86_64 with LTO, `codegen-units = 1`, `panic = "abort"`, and stripping enabled. The resulting binary contains no WGPU components.

The renderer configuration used by v1.5.5 is the same one that passed local runtime checks for audio playback, album artwork, mosaic rendering, and SVG rendering. Validation remains local, not a cross-platform test matrix; no FPS or speed improvement is claimed.

## Cross-platform compatibility

For v1.5.5, cross-platform compatibility is expected but not confirmed. I have not tested Windows, macOS, or other Linux distributions for this release, so no measured compatibility rate is claimed.

The renderer change is still limited to configuration: I enabled Iced's TinySkia feature and removed the explicit Iced `wgpu` feature. I left the upstream audio, providers, playback, media controls, file handling, and platform-specific application code alone. The upstream v1.5.2–v1.5.5 integration comes from GooseMusic upstream. The existing X11 and Wayland features are still enabled.

Basically, I changed how the interface is drawn, not how the app plays music or connects to providers. The build and runtime checks passed on Linux x86_64. Compatibility on Windows, macOS, and other Linux distributions is expected, but not confirmed.

## Known issue

Very long track titles can sometimes overlap the next row in the upright track list. It is a small visual issue and was left as-is.

## Build

You need the Rust toolchain.

```bash
git clone --branch release-prep/tiny-skia-v1.5.5 https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr.git
cd music_plr
cargo build --release --locked
```

The binary is in `target/release/`.

Prebuilt `v1.5.5-tiny-skia.1` assets are available on [GitHub Releases](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/releases), including Linux and macOS install assets and the Windows installer.

## Links

- [TinySkia v1.5.5 branch](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/release-prep/tiny-skia-v1.5.5)
- [TinySkia v1.5.5 migration commit 6b88926](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/commit/6b88926036a1e957a0b2c588be784cccdace2908)
- [Release v1.5.5-tiny-skia.1](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/releases/tag/v1.5.5-tiny-skia.1)
- [Original GooseMusic project](https://github.com/GooseOb/music_plr)
