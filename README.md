# GooseMusic TinySkia

This is my TinySkia fork of GooseMusic. It is currently based on GooseMusic v1.5.5 and uses Iced's TinySkia renderer instead of WGPU.

The v1.5.5 renderer changes are on the [release-prep/tiny-skia-v1.5.5 branch](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/release-prep/tiny-skia-v1.5.5).

## Changes

- Based on GooseMusic `v1.5.5`.
- Uses Iced `0.14`.
- Uses `iced_tiny_skia 0.14.1` and `tiny-skia 0.11.4`.
- Enables Iced's `tiny-skia` feature and removes its explicit `wgpu` feature.
- Keeps the `image`, `svg`, `tokio`, `canvas`, `advanced`, `crisp`, `web-colors`, `thread-pool`, `x11`, and `wayland` features.
- Updates `Cargo.lock`.
- Includes the upstream v1.5.2–v1.5.5 changes, including the InnerTube migration (replacing the Python/`ytmusicapi` search path) and multi-author tracks.
- Keeps the fork's GitHub Releases updater and cross-platform release packaging.

The renderer change removes WGPU from the active dependency graph. The v1.5.5 lockfile diff has 13 insertions and 315 deletions; some inactive WGPU-named package records remain, but no WGPU crate is in the active dependency graph or release binary.

The release workflow builds Linux, macOS, and Windows assets. It also creates SHA-256 sidecars, macOS DMGs, and a Windows Inno Setup installer. GitHub Releases are created as drafts.

## Memory usage

In my comparison, normal GooseMusic used **122 MiB RSS and 122 MiB PSS**. This TinySkia build used **44 MiB RSS and 44 MiB PSS** — roughly **64% less** in both measurements.

## Build and testing

The v1.5.5 branch passes `cargo metadata --locked --no-deps`, `cargo check --locked`, and the fork updater tests. I also built the optimized release binary successfully on Linux x86_64 with LTO, one codegen unit, panic abort, and stripping enabled.

I tested audio playback, album artwork, mosaic rendering, and SVG rendering locally. The release binary has no WGPU components. I haven't run a Windows or macOS test yet, so compatibility on those platforms isn't confirmed. I also haven't measured FPS or claimed a speed improvement.

## Cross-platform compatibility

The existing X11 and Wayland features are still enabled. I only changed the renderer configuration; the upstream audio, providers, playback, media controls, file handling, and platform-specific application code were left alone. The upstream v1.5.2–v1.5.5 changes come from GooseMusic itself.

In short, the interface is drawn with TinySkia now. The rest of the app is still GooseMusic. Builds and runtime checks have passed on Linux x86_64, but other platforms still need testing.

## Known issue

Very long track titles can overlap the next row in the upright track list.

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
